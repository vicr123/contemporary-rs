//! Configuration loading and parsing for the Contemporary i18n system.
//!
//! This module handles loading the `i18n.toml` configuration file and provides
//! types representing the configuration options.
//!
//! # Configuration file
//!
//! The i18n system is configured via an `i18n.toml` file in your project root:
//!
//! ```toml
//! [i18n]
//! default_language = "en"           # Source language (default: "en")
//! translation_directory = "translations"  # Where translation files are stored
//! match_line_endings = true         # Normalize line endings to platform default
//! ```
//!
//! If no configuration file exists, the defaults above are used.

use std::{
    fs::{OpenOptions, create_dir_all},
    io::Read,
    path::{Path, PathBuf},
};

use serde::Deserialize;

/// Load the i18n configuration from the project's `i18n.toml` file.
///
/// If no configuration file exists, returns a [`Config`] with default values.
///
/// # Arguments
///
/// * `project_root` - Path to the directory containing the project's `Cargo.toml`
///
/// # Panics
///
/// Panics if:
/// - The configuration file exists but cannot be read
/// - The configuration file contains invalid TOML
/// - The `default_language` is empty or whitespace-only
///
/// # Example
///
/// ```rust,ignore
/// use std::path::Path;
/// use cntp_i18n_build_core::config::get_i18n_config;
///
/// let config = get_i18n_config(Path::new("/path/to/project"));
/// println!("Default language: {}", config.i18n.default_language);
/// ```
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

/// Top-level configuration structure.
///
/// This is deserialized from the `i18n.toml` configuration file.
#[derive(Default, Deserialize)]
pub struct Config {
    /// The i18n-specific configuration section.
    #[serde(default)]
    pub i18n: I18n,
}

/// Configuration options for the i18n system.
///
/// This corresponds to the `[i18n]` section in `i18n.toml`.
#[derive(Deserialize)]
#[serde(default)]
pub struct I18n {
    /// The default/source language for translations.
    ///
    /// This should be an ISO-639 language code, optionally with a region
    /// (e.g., "en", "en-US", "pt-BR").
    ///
    /// Default: `"en"`
    pub default_language: String,

    /// The directory where translation files are stored, relative to `Cargo.toml`.
    ///
    /// Default: `"translations"`
    translation_directory: PathBuf,

    /// Whether to normalize line endings in translation strings.
    ///
    /// When `true`, line endings are converted to the target platform's
    /// native format (`\n` on Unix, `\r\n` on Windows).
    ///
    /// Default: `true`
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
    /// Get the absolute path to the translation directory.
    ///
    /// Creates the directory if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `manifest_directory` - Path to the directory containing `Cargo.toml`
    ///
    /// # Panics
    ///
    /// Panics if the directory cannot be created.
    pub fn translation_directory(&self, manifest_directory: &Path) -> PathBuf {
        let file_path = manifest_directory.join(&self.translation_directory);
        create_dir_all(&file_path).expect("Unable to create translations directory");
        file_path
    }

    /// Get all translation catalog files (excluding `meta.json`).
    ///
    /// Returns paths to all `.json` files in the translation directory,
    /// excluding the metadata file.
    ///
    /// # Arguments
    ///
    /// * `manifest_directory` - Path to the directory containing `Cargo.toml`
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

    /// Get the path to the default language's translation catalog file.
    ///
    /// For example, if `default_language` is "en", this returns
    /// `<translation_directory>/en.json`.
    ///
    /// # Arguments
    ///
    /// * `manifest_directory` - Path to the directory containing `Cargo.toml`
    pub fn translation_catalog_file(&self, manifest_directory: &Path) -> PathBuf {
        self.translation_directory(manifest_directory)
            .join(format!("{}.json", self.default_language))
    }

    /// Get the path to the translation metadata file (`meta.json`).
    ///
    /// The metadata file contains information about each translation string,
    /// such as where it was defined and whether it's plural.
    ///
    /// # Arguments
    ///
    /// * `manifest_directory` - Path to the directory containing `Cargo.toml`
    pub fn translation_meta_file(&self, manifest_directory: &Path) -> PathBuf {
        self.translation_directory(manifest_directory)
            .join("meta.json")
    }
}
