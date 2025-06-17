pub use contemporary_i18n_macros::{tr, tr_load, trn};
use fxhash::FxHashMap;
use once_cell::sync::Lazy;
use std::{fmt::Display, sync::RwLock};

pub use contemporary_i18n_core::{I18nEntry, I18nPluralStringEntry, I18nSource, I18nStringEntry};

pub use locale_config::{LanguageRange, Locale};

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

    pub fn lookup(&self, key: &str, variables: Option<FxHashMap<String, Variable>>) -> String {
        for source in &self.sources {
            let Some(entry) = source.lookup(&self.locale, key) else {
                continue;
            };

            let variables = variables.unwrap_or(FxHashMap::default());

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

    pub fn quote_string(&self, string: impl Display) -> String {
        format!("\"{}\"", string.to_string())
    }
}
