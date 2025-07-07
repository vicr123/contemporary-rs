use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, ErrorKind},
    path::Path,
};

use serde_json::Value;

pub enum TranslationEntry {
    Entry(String),
    PluralEntry(HashMap<String, String>),
}

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
