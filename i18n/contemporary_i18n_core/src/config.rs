use std::{
    fs::OpenOptions,
    io::Read,
    path::{Path, PathBuf},
};

use serde::Deserialize;

pub fn get_i18n_config(project_root: &Path) -> Config {
    let mut config_path = PathBuf::from(project_root);
    config_path.push("i18n.toml");

    let config = if config_path.exists() {
        let mut file = OpenOptions::new().read(true).open(&config_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).expect("unable to read i18n configuration")
    } else {
        Config::default()
    };

    if config.i18n.default_language.trim().is_empty() {
        panic!("i18n configuration default language is empty")
    }

    config
}

#[derive(Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub i18n: I18n,
}

#[derive(Deserialize)]
#[serde(default)]
pub struct I18n {
    pub default_language: String,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            default_language: "en".into(),
        }
    }
}
