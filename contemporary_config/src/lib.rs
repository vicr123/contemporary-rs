pub mod config;

use crate::config::ContemporaryConfigApplication;
use serde::Deserialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ContemporaryConfig {
    pub application: ContemporaryConfigApplication,
}

impl ContemporaryConfig {
    pub fn new_from_path(path: PathBuf) -> Option<Self> {
        if path.exists() {
            let Ok(mut file) = OpenOptions::new().read(true).open(&path) else {
                return None;
            };
            let mut contents = String::new();
            let Ok(_) = file.read_to_string(&mut contents) else {
                return None;
            };

            let Ok(config) = toml::from_str(&contents) else {
                return None;
            };

            config
        } else {
            None
        }
    }

    pub fn new_from_build_env() -> Option<Self> {
        let Ok(cargo_manifest_dir) = env::var("CARGO_MANIFEST_DIR") else {
            return None;
        };

        let manifest_dir = PathBuf::from(cargo_manifest_dir);
        let contemporary_path = manifest_dir.join("Contemporary.toml");
        Self::new_from_path(contemporary_path)
    }
}
