pub use cntp_i18n_macros::{tr, tr_load, trn};
use once_cell::sync::Lazy;
use std::sync::RwLock;

pub use cntp_i18n_core::{
    string::I18nString, I18nEntry, I18nPluralStringEntry, I18nSource, I18nStringEntry,
};
pub use cntp_localesupport::locale_formattable::LocaleFormattable;
pub use cntp_localesupport::Locale;
pub use phf;

pub static I18N_MANAGER: Lazy<RwLock<I18nManager>> =
    Lazy::new(|| RwLock::new(I18nManager::default()));

#[macro_export]
macro_rules! i18n_manager {
    () => {
        cntp_i18n::I18N_MANAGER.read().unwrap()
    };
}

pub struct I18nManager {
    sources: Vec<Box<dyn I18nSource>>,
    pub locale: Locale,
}

pub enum Variable {
    String(String),
    Count(isize),
}

type LookupVariable<'a> = &'a (&'a str, Variable);

impl I18nManager {
    pub fn load_source(&mut self, source: impl I18nSource + 'static) {
        self.sources.push(Box::new(source));
    }

    pub fn lookup<'a, T>(&self, key: &str, variables: &'a T, lookup_crate: &str) -> I18nString
    where
        &'a T: IntoIterator<Item = LookupVariable<'a>>,
    {
        for source in &self.sources {
            let Some(entry) = source.lookup(&self.locale, key, lookup_crate) else {
                continue;
            };

            // TODO: Cache the resolved string
            let mut resolved = match &entry {
                I18nEntry::Entry(entry) => entry.entry.clone(),
                I18nEntry::PluralEntry(entry) => {
                    let (_, count) = (variables)
                        .into_iter()
                        .find(|(name, _)| *name == "count")
                        .unwrap_or_else(|| {
                            panic!(
                                "Resolved plural string for {key}, but no count variable provided \
                                for substitution",
                            )
                        });

                    match count {
                        Variable::Count(count) => entry.lookup(*count),
                        Variable::String(string) => {
                            panic!("Count variable ({string}) not of type isize")
                        }
                    }
                }
            };

            // If the translation is empty, fall back to the next source
            if resolved.is_empty() {
                continue;
            }

            // Substitute the variables
            for (name, substitution) in variables.into_iter() {
                if *name == "count" {
                    if entry.is_singular() {
                        panic!(
                            "Resolved non-plural string for {key}, but count variable provided \
                            for substitution",
                        )
                    }

                    // Special case the count variable which should be handled in a plural entry
                    continue;
                }

                resolved = match substitution {
                    Variable::Count(count) => {
                        panic!("Substitution variable ({name}) not of type string (is {count})",)
                    }
                    Variable::String(string) => resolved
                        .replace(format!("{{{{{name}}}}}",).as_str(), string.as_str())
                        .into(),
                }
            }

            return resolved;
        }

        // None of the translation sources we have were able to find a key so just return the key
        key.to_string().into()
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        I18nManager {
            sources: vec![],
            locale: Locale::current(),
        }
    }
}
