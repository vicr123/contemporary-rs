use anyhow::anyhow;
use regex::Regex;
use std::os::unix::process::ExitStatusExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use tracing::error;

pub struct DiskImage {
    pub disk_image_path: PathBuf,
}

pub struct DiskImageMount {
    pub mount_point: PathBuf,
}

impl DiskImage {
    pub fn new(size: u64, name: &str, output: PathBuf, filesystem: &str) -> anyhow::Result<Self> {
        let hdiutil_status = Command::new("hdiutil")
            .arg("create")
            .arg(&output)
            .arg("-ov")
            .arg("-fs")
            .arg(filesystem)
            .arg("-size")
            .arg(format!("{size}"))
            .arg("-volname")
            .arg(name)
            .status()?;

        if !hdiutil_status.success() {
            return Err(anyhow!(
                "hdiutil create failed: hdiutil status: {:?}",
                hdiutil_status
            ));
        }

        Ok(DiskImage {
            disk_image_path: output,
        })
    }

    pub fn new_by_path(path: PathBuf) -> Self {
        DiskImage {
            disk_image_path: path,
        }
    }

    pub fn convert(&self, output: PathBuf, format: &str) -> anyhow::Result<DiskImage> {
        let hdiutil_status = Command::new("hdiutil")
            .arg("convert")
            .arg(&self.disk_image_path)
            .arg("-ov")
            .arg("-format")
            .arg(format)
            .arg("-imagekey")
            .arg("zlib-level=9")
            .arg("-o")
            .arg(&output)
            .status()?;

        if !hdiutil_status.success() {
            return Err(anyhow!(
                "hdiutil convert failed: hdiutil status: {:?}",
                hdiutil_status
            ));
        }

        Ok(DiskImage {
            disk_image_path: output,
        })
    }

    pub fn mount(&self) -> anyhow::Result<DiskImageMount> {
        let output = Command::new("hdiutil")
            .arg("attach")
            .arg(&self.disk_image_path)
            .arg("-nobrowse")
            .arg("-noverify")
            .arg("-noautoopen")
            .arg("-noautofsck")
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Unable to mount the disk image"));
        };

        let output = str::from_utf8(&output.stdout)?;
        let mount_path_regex = Regex::new(r"(?m)(/Volumes/.+)$")?;

        let mount_point = mount_path_regex
            .captures(output)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
            .ok_or_else(|| anyhow!("Unable to locate the mount point of the disk image"))?;

        Ok(DiskImageMount {
            mount_point: PathBuf::from(mount_point),
        })
    }
}

impl Drop for DiskImageMount {
    fn drop(&mut self) {
        for attempts in 0..8 {
            let Ok(exit_status) = Command::new("hdiutil")
                .args({
                    if attempts == 8 {
                        vec![
                            "detach",
                            &self.mount_point.as_os_str().to_str().unwrap(),
                            "-force",
                        ]
                    } else {
                        vec!["detach", &self.mount_point.as_os_str().to_str().unwrap()]
                    }
                })
                .status()
            else {
                return;
            };

            if exit_status.into_raw() != 16 {
                return;
            }

            sleep(Duration::from_secs(2_i32.pow(attempts) as u64));
        }

        // Okay, let's force detach now
        error!("Unable to detach the disk image. The disk image may not be created correctly.");
    }
}
