use std::{
    fs::{OpenOptions, create_dir_all},
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
    translation_directory: PathBuf,
    pub match_line_endings: bool,
}

impl Default for I18n {
    fn default() -> Self {
        Self {
            default_language: "en".into(),
            translation_directory: "translations".into(),
            match_line_endings: true,
        }
    }
}

impl I18n {
    pub fn translation_directory(&self, manifest_directory: &Path) -> PathBuf {
        let file_path = manifest_directory.join(&self.translation_directory);
        create_dir_all(&file_path).expect("Unable to create translations directory");
        file_path
    }

    pub fn catalog_files(&self, manifest_directory: &Path) -> Vec<PathBuf> {
        let dir_contents = self
            .translation_directory(manifest_directory)
            .read_dir()
            .unwrap();
        dir_contents
            .enumerate()
            .filter(|(_size, entry)| {
                entry.as_ref().is_ok_and(|entry| {
                    entry.metadata().is_ok_and(|meta| meta.is_file())
                        && entry.file_name().to_str().unwrap().ends_with(".json")
                        && entry.file_name().to_str().unwrap() != "meta.json"
                })
            })
            .map(|(_size, entry)| entry.unwrap().path())
            .collect()
    }

    pub fn translation_catalog_file(&self, manifest_directory: &Path) -> PathBuf {
        self.translation_directory(manifest_directory)
            .join(format!("{}.json", self.default_language))
    }

    pub fn translation_meta_file(&self, manifest_directory: &Path) -> PathBuf {
        self.translation_directory(manifest_directory)
            .join("meta.json")
    }
}
