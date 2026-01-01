//! Translation file loading utilities.
//!
//! This module provides functions for loading translation files from disk.
//! It is used by the macro system to read translation catalogs at compile time.
//!
//! ## File format
//!
//! Translation files are JSON objects mapping keys to either:
//! - A string (for simple translations)
//! - An object with plural categories (for plural translations)
//!
//! ### Simple translations
//!
//! ```json
//! {
//!     "HELLO": "Hello!",
//!     "GOODBYE": "Goodbye!"
//! }
//! ```
//!
//! ### Plural translations
//!
//! ```json
//! {
//!     "ITEMS": {
//!         "one": "{{count}} item",
//!         "other": "{{count}} items"
//!     }
//! }
//! ```

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, ErrorKind},
    path::Path,
};

use serde_json::Value;

/// A translation entry loaded from a translation file.
///
/// This enum represents the two types of translations:
/// - Simple string translations
/// - Plural translations with multiple forms
pub enum TranslationEntry {
    /// A simple, non-plural translation string.
    Entry(String),
    /// A plural translation with forms keyed by plural category.
    ///
    /// The keys are ICU plural categories: "zero", "one", "two", "few", "many", "other".
    /// The "other" key is always required.
    PluralEntry(HashMap<String, String>),
}

/// Load translations from a JSON file.
///
/// Reads a translation catalog file and returns all translation entries as a
/// vector of key-value pairs.
///
/// # Arguments
///
/// * `path` - Path to the JSON translation file
///
/// # Returns
///
/// A vector of `(key, entry)` tuples, where each entry is either a simple
/// string or a plural entry with multiple forms.
///
/// # Errors
///
/// Returns an error if:
/// - The file cannot be opened or read
/// - The file is not valid JSON
/// - The JSON structure is invalid (not an object, or contains invalid entries)
/// - A plural entry is missing the required "other" form
///
/// # Example
///
/// ```rust,ignore
/// use std::path::Path;
/// use cntp_i18n_build_core::load::{translation, TranslationEntry};
///
/// let entries = translation(Path::new("translations/en.json"))?;
/// for (key, entry) in entries {
///     match entry {
///         TranslationEntry::Entry(text) => println!("{}: {}", key, text),
///         TranslationEntry::PluralEntry(forms) => {
///             println!("{}: {} forms", key, forms.len());
///         }
///     }
/// }
/// ```
pub fn translation(path: &Path) -> io::Result<Vec<(String, TranslationEntry)>> {
    let file = OpenOptions::new().read(true).open(path)?;
    let value: Value = serde_json::from_reader(file)?;

    let object = value
        .as_object()
        .ok_or(io::Error::new(ErrorKind::InvalidData, "not an object"))?;

    let mut entries = vec![];

    for kv in object {
        let name = kv.0.clone();

        if let Some(string) = kv.1.as_str() {
            entries.push((name, TranslationEntry::Entry(string.to_string())));
        } else if let Some(inner_object) = kv.1.as_object() {
            let other = inner_object.get("other");
            if other.is_none() {
                return Err(io::Error::new(
                    ErrorKind::InvalidData,
                    r#"Plural entry has no "other" entry"#,
                ));
            }

            let squeeze = inner_object
                .into_iter()
                .filter(|(cat, _)| {
                    *cat == "zero"
                        || *cat == "one"
                        || *cat == "two"
                        || *cat == "few"
                        || *cat == "many"
                        || *cat == "other"
                })
                .filter_map(|(cat, v)| v.as_str().map(|string| (cat.clone(), string.to_string())))
                .collect();

            entries.push((name, TranslationEntry::PluralEntry(squeeze)))
        } else {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "key with invalid value",
            ));
        }
    }

    Ok(entries)
}
