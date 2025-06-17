pub use contemporary_i18n_macros::{tr, trn};
use fxhash::FxHashMap;
use std::collections::HashMap;
use std::fmt::Display;

#[cfg(feature = "gpui")]
use gpui::Global;

use contemporary_i18n_core::{I18nEntry, I18nSource};

use locale_config::Locale;

pub struct I18nManager {
    sources: Vec<Box<dyn I18nSource>>,
    locale: Locale,
}

pub enum Variable {
    String(String),
    Count(isize),
}

impl I18nManager {
    pub fn new() -> I18nManager {
        I18nManager {
            sources: vec![],
            locale: Locale::current(),
        }
    }

    pub fn lookup(&self, key: &str, variables: FxHashMap<String, Variable>) -> String {
        for source in &self.sources {
            let Some(entry) = source.lookup(key) else {
                continue;
            };

            // TODO: Cache the resolved string
            let mut resolved = match &entry {
                I18nEntry::Entry(entry) => entry.entry.clone(),
                I18nEntry::PluralEntry(entry) => {
                    let count = variables.get("count").expect(
                        format!(
                            "Resolved plural string for {}, but no count variable provided for \
                            substitution",
                            key
                        )
                        .as_str(),
                    );

                    match count {
                        Variable::Count(count) => entry.lookup(&self.locale, *count),
                        Variable::String(string) => {
                            panic!("Count variable ({}) not of type isize", string)
                        }
                    }
                }
            };

            // Substitute the variables
            for (name, substitution) in variables {
                if name == "count" {
                    if entry.is_singular() {
                        panic!(
                            "Resolved non-plural string for {}, but count variable provided \
                            for substitution",
                            key
                        )
                    }

                    // Special case the count variable which should be handled in a plural entry
                    continue;
                }

                resolved = match substitution {
                    Variable::Count(count) => panic!(
                        "Substitution variable ({}) not of type string (is {})",
                        name, count
                    ),
                    Variable::String(string) => {
                        resolved.replace(format!("{{{{{}}}}}", name).as_str(), string.as_str())
                    }
                }
            }

            return resolved;
        }

        // None of the translation sources we have were able to find a key so just return the key
        key.to_string()
    }

    pub fn quote_string(&self, string: impl Display) -> String {
        format!("\"{}\"", string.to_string())
    }
}

#[cfg(feature = "gpui")]
impl Global for I18nManager {}
