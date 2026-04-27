use crate::tool_setup::ToolSetup;
use isahc::config::{Configurable, RedirectPolicy};
use isahc::{Request, RequestExt};
use std::env::consts::ARCH;
use std::fs::{File, Permissions, set_permissions};
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::process::{Command, exit};
use tempfile::TempDir;
use tracing::{error, info};

pub fn deploy_appimage(setup_data: &ToolSetup, output_file: &str) {
    let appdir_root = setup_data.output_directory.join("appdir");
    if !appdir_root.exists() {
        error!("AppDir does not exist. Please bundle first.");
        exit(1);
    }

    let url = format!(
        "https://github.com/AppImage/appimagetool/releases/download/1.9.0/appimagetool-{ARCH}.AppImage"
    );
    info!("Downloading appimagetool from URL {}", url);

    // Create a temporary directory
    let temp_dir = TempDir::new().expect("Failed to create temporary directory");
    let appimagetool_path = temp_dir.path().join("appimagetool.AppImage");

    let Ok(mut appimagetool_file) = File::create(&appimagetool_path) else {
        error!("Failed to create appimagetool file");
        exit(1);
    };

    // Configure and start download
    let Ok(mut response) = Request::get(url)
        .redirect_policy(RedirectPolicy::Follow)
        .body(())
        .unwrap()
        .send()
    else {
        error!("Failed to download appimagetool");
        exit(1);
    };

    if !response.status().is_success() {
        error!("Failed to download appimagetool");
        error!(
            "The server returned the status: {} {}",
            response.status().as_str(),
            response.status().canonical_reason().unwrap_or("")
        );
        exit(1);
    }

    let total_size = response
        .headers()
        .get("Content-Length")
        .and_then(|ct_len| ct_len.to_str().ok())
        .and_then(|ct_len| ct_len.parse::<u64>().ok())
        .unwrap_or(0);

    let mut _downloaded: u64 = 0;
    let mut buffer = [0; 8192];

    // Download with progress updates
    while let Ok(n) = response.body_mut().read(&mut buffer) {
        if n == 0 {
            break;
        }
        let Ok(_) = appimagetool_file.write_all(&buffer[..n]) else {
            error!("Unable to write appimagetool to disk.");
            exit(1);
        };
        _downloaded += n as u64;

        // TODO: Print progress to the console periodically
    }

    info!("Downloaded {} bytes", total_size);
    drop(appimagetool_file);

    let Ok(_) = set_permissions(&appimagetool_path, Permissions::from_mode(0o755)) else {
        error!("Failed to set permissions on appimagetool");
        exit(1);
    };

    info!("Running appimagetool to create AppImage");

    // Use appimagetool to write the AppImage to the output file
    let command_result = Command::new(&appimagetool_path)
        .arg(appdir_root)
        .arg(output_file)
        .spawn();
    let Ok(mut appimagetool_process) = command_result else {
        let e = command_result.unwrap_err();
        error!("Failed to run appimagetool: {e}");
        exit(1);
    };

    let Ok(status) = appimagetool_process.wait() else {
        error!("Failed to wait for appimagetool");
        exit(1);
    };

    if !(status.success()) {
        error!("AppImage creation failed");
        exit(1);
    }
    info!("AppImage created successfully");
}
