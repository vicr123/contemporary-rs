use crate::self_update::UpdateInformation;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use gpui::http_client::anyhow;
use gpui::private::anyhow;
use isahc::http::{StatusCode, Uri};
use isahc::{AsyncReadResponseExt, HttpClient};
use minisign_verify::{PublicKey, Signature};
use serde::Deserialize;
use smol::fs;
use smol::fs::File;
use smol::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use smol::stream::StreamExt;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;
use url::Url;

pub struct BinChickenClient {
    working_directory: PathBuf,
    url: Url,
    repository: &'static str,
    our_uuid: &'static str,
    signature_public_key: PublicKey,

    client: HttpClient,
}

#[derive(Deserialize)]
struct Artifact {
    number: u64,
    version: Option<String>,
}

impl BinChickenClient {
    pub fn new(
        working_directory: PathBuf,
        url: Url,
        repository: &'static str,
        our_uuid: &'static str,
        signature_public_key: PublicKey,
    ) -> Self {
        let client = HttpClient::new().unwrap();
        Self {
            working_directory,
            url,
            repository,
            our_uuid,
            signature_public_key,

            client,
        }
    }
    
    pub fn artifact_local_path(&self, artifact_number: u64) -> PathBuf {
        self.working_directory.join(format!("update-{}", artifact_number))
            .join("update-data.bin")
    }

    pub async fn check_for_updates(&self) -> Result<Option<UpdateInformation>, anyhow::Error> {
        let update_check_url = Uri::from_str(
            format!(
                "{}api/repositories/{}/latest/by_uuid/{}",
                self.url, self.repository, self.our_uuid
            )
            .as_str(),
        )?;

        let x = update_check_url.to_string();

        let mut response = self.client.get_async(update_check_url).await?;

        match response.status() {
            StatusCode::OK => {
                let response = response.json::<Artifact>().await?;

                // Ok, we're ready to update.
                Ok(Some(UpdateInformation {
                    artifact_number: response.number,
                    artifact_version: response.version,
                }))
            }
            StatusCode::NO_CONTENT => {
                // We are up to date
                // TODO: Remove any old update data
                Ok(None)
            }
            _ => Err(anyhow!("Unexpected status code: {}", response.status())),
        }
    }

    pub async fn download_artifact(&self, artifact_number: u64) -> Result<(), anyhow::Error> {
        let artifact_url = Uri::from_str(
            format!(
                "{}api/repositories/{}/artifacts/{}",
                self.url, self.repository, artifact_number
            )
            .as_str(),
        )?;

        info!("Update available: Artifact number {}", artifact_number);
        let _ = fs::create_dir_all(&self.working_directory).await;

        let mut dir = fs::read_dir(&self.working_directory).await?;
        while let Some(Ok(entry)) = dir.next().await {
            let file_name = entry.file_name().to_str().unwrap().to_string();
            if file_name.starts_with("update-")
                && entry.file_type().await?.is_dir()
                && file_name != format!("update-{}", artifact_number)
            {
                info!("Removing old update directory {}", file_name);
                let _ = fs::remove_dir_all(entry.path()).await;
            }
        }

        let update_directory = self
            .working_directory
            .join(format!("update-{}", artifact_number));

        let mut download_required = true;
        if update_directory.exists() {
            if self.verify_artifact(artifact_number).await.is_err() {
                // Remove the old update directory and download again
                info!("Update data is corrupt, removing old update directory");
                let _ = fs::remove_dir_all(&update_directory).await;
            } else {
                download_required = false;
            }
        }

        if download_required {
            // Download the new update data
            info!("Downloading update data");

            fs::create_dir_all(&update_directory).await?;

            let update_data_file = update_directory.join("update-data.bin");
            let update_signature_file = update_directory.join("update-data.bin.minisig");

            let response = self.client.get_async(artifact_url).await?;

            if response.status() != StatusCode::OK {
                return Err(anyhow!("Unexpected status code: {}", response.status()));
            }

            let signature_header = response
                .headers()
                .get("X-Bin-Chicken-Signature")
                .ok_or(anyhow!("No X-Bin-Chicken-Signature header found"))?;
            let decoded_signature = BASE64_STANDARD.decode(signature_header.as_bytes())?;
            fs::write(&update_signature_file, decoded_signature).await?;

            let output_file = File::create(&update_data_file).await?;

            if let Some(len) = response.body().len() {
                info!("Update package is {} bytes long", len);
                output_file.set_len(len).await?;
            }

            let mut output_file_write = BufWriter::new(output_file);

            let mut bytes_stream = response.into_body().bytes();
            while let Some(Ok(bytes)) = bytes_stream.next().await {
                output_file_write.write_all(&[bytes]).await?;
            }

            output_file_write.flush().await?;
            output_file_write.close().await?;

            // Check the update data again
            self.verify_artifact(artifact_number).await?;
            info!("Update data verified");
        }

        Ok(())
    }

    pub async fn verify_artifact(&self, artifact_number: u64) -> Result<(), anyhow::Error> {
        let update_directory = self
            .working_directory
            .join(format!("update-{}", artifact_number));

        let update_data_file = update_directory.join("update-data.bin");
        let update_signature_file = update_directory.join("update-data.bin.minisig");

        let signature = fs::read_to_string(&update_signature_file).await?;
        let signature = Signature::decode(&signature)?;

        let mut update_data = File::open(&update_data_file).await?;
        let mut verifier = self.signature_public_key.verify_stream(&signature)?;
        let mut buf = [0_u8; 2048];
        loop {
            let read = update_data.read(&mut buf).await?;
            if read == 0 {
                break;
            }
            verifier.update(&buf[..read]);
        }
        verifier.finalize()?;

        Ok(())
    }
}
