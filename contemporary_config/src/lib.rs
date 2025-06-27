pub mod config;

use std::collections::HashMap;
use crate::config::{ContemporaryConfigApplication, ContemporaryConfigDeployment};
use serde::Deserialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ContemporaryConfig {
    pub application: ContemporaryConfigApplication,
    pub deployment: ContemporaryConfigDeployment,
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

    pub fn deployment(&self, arch: &str) -> ContemporaryConfigDeployment {
        let deployment = self.deployment.clone();
        let specific_deployment = if let Some(x) = self.deployment.clone().children.get(arch) {
            x.clone()
        } else {
            ContemporaryConfigDeployment::new()
        };

        ContemporaryConfigDeployment {
            application_name: specific_deployment.application_name.or(deployment.application_name),
            apple_localisation_dir: specific_deployment.apple_localisation_dir.or(deployment.apple_localisation_dir),
            desktop_entry: specific_deployment.desktop_entry.or(deployment.desktop_entry),
            icon: specific_deployment.icon.or(deployment.icon),
            contemporary_base_icon: specific_deployment.contemporary_base_icon.or(deployment.contemporary_base_icon),
            children: HashMap::new()
        }
    }
}
