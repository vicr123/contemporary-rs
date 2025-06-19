pub use contemporary_i18n_macros::{tr, tr_load, trn};
use once_cell::sync::Lazy;
use std::{fmt::Display, sync::RwLock};

pub use contemporary_i18n_core::{I18nEntry, I18nPluralStringEntry, I18nSource, I18nStringEntry};
pub use contemporary_localesupport::Locale;

pub static I18N_MANAGER: Lazy<RwLock<I18nManager>> = Lazy::new(|| RwLock::new(I18nManager::new()));

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

    pub fn load_source(&mut self, source: impl I18nSource + 'static) {
        self.sources.push(Box::new(source));
    }

    pub fn lookup(&self, key: &str, variables: Vec<(&str, Variable)>) -> String {
        for source in &self.sources {
            let Some(entry) = source.lookup(&self.locale, key) else {
                continue;
            };

            // TODO: Cache the resolved string
            let mut resolved = match &entry {
                I18nEntry::Entry(entry) => entry.entry.clone(),
                I18nEntry::PluralEntry(entry) => {
                    let (_, count) = variables.iter().find(|(name, _)| *name == "count").expect(
                        format!(
                            "Resolved plural string for {}, but no count variable provided for \
                            substitution",
                            key
                        )
                        .as_str(),
                    );

                    match count {
                        Variable::Count(count) => entry.lookup(*count),
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
}
