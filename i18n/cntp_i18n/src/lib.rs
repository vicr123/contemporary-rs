#![warn(missing_docs)]

pub use cntp_i18n_macros::{tr, tr_load, tr_noop, trn, trn_noop};
use cntp_localesupport::modifiers::ModifierVariable;
use once_cell::sync::Lazy;
use quick_cache::sync::Cache;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};
use std::sync::RwLock;

pub use cntp_i18n_core::{
    I18nEntry, I18nPluralStringEntry, I18nSource, I18nStringEntry, I18nStringPart,
    string::I18nString,
};
pub use cntp_localesupport::locale_formattable::LocaleFormattable;
pub use cntp_localesupport::modifiers::{Date, Quote, StringModifier};
pub use cntp_localesupport::{LayoutDirection, Locale};
pub use phf;

/// The global i18n manager.
pub static I18N_MANAGER: Lazy<RwLock<I18nManager>> =
    Lazy::new(|| RwLock::new(I18nManager::default()));

/// Gets the global i18n manager.
#[macro_export]
macro_rules! i18n_manager {
    () => {
        cntp_i18n::I18N_MANAGER.read().unwrap()
    };
}

/// Manages the state of the i18n system in the app.
///
/// The i18n manager is responsible for keeping
/// track of all the loaded translation files, as well as the current locale settings.
pub struct I18nManager {
    sources: Vec<Box<dyn I18nSource>>,
    pub locale: Locale,
    cache: Cache<u64, I18nString>,
}

pub trait ErasedStringModifierTransform {
    fn transform(&self, locale: &Locale) -> String;
    fn hash(&self, state: &mut FxHasher);
}

pub struct BaseStringModifierInvocation<'a, T: ?Sized + Hash>(
    &'a dyn StringModifier<&'a T>,
    &'a [ModifierVariable<'a>],
    &'a T,
);

impl<'a, T: ?Sized + Hash> BaseStringModifierInvocation<'a, T> {
    pub fn new(
        modifier: &'a dyn StringModifier<&'a T>,
        variables: &'a [ModifierVariable<'a>],
        input: &'a T,
    ) -> Self {
        BaseStringModifierInvocation(modifier, variables, input)
    }
}

impl<'a, T: ?Sized + Hash> ErasedStringModifierTransform for BaseStringModifierInvocation<'a, T> {
    fn transform(&self, locale: &Locale) -> String {
        let BaseStringModifierInvocation(modifier, variables, input) = self;
        modifier.transform(locale, input, variables)
    }

    fn hash(&self, state: &mut FxHasher) {
        let BaseStringModifierInvocation(_, _, input) = self;
        input.hash(state);
    }
}

pub struct SubsequentStringModifierInvocation<'a>(
    &'a dyn StringModifier<String>,
    &'a [ModifierVariable<'a>],
);

impl<'a> SubsequentStringModifierInvocation<'a> {
    pub fn new(
        modifier: &'a dyn StringModifier<String>,
        variables: &'a [ModifierVariable<'a>],
    ) -> Self {
        SubsequentStringModifierInvocation(modifier, variables)
    }
}

pub enum Variable<'a> {
    Modified(
        &'a dyn ErasedStringModifierTransform,
        &'a [SubsequentStringModifierInvocation<'a>],
    ),
    String(String),
    Count(isize),
}

impl Variable<'_> {
    fn hash_value(&self, state: &mut FxHasher) {
        match self {
            Variable::Modified(modifier, _) => {
                modifier.hash(state);
            }
            Variable::String(string) => string.hash(state),
            Variable::Count(count) => count.hash(state),
        }
    }
}

type LookupVariable<'a> = &'a (&'a str, Variable<'a>);

impl I18nManager {
    /// Load a translation file into the manager.
    ///
    /// Example:
    /// ```rs
    /// i18n_manager!().load_source(tr_load!());
    /// ```
    pub fn load_source(&mut self, source: impl I18nSource + 'static) {
        self.sources.push(Box::new(source));
    }

    /// Lookup a translation from the cache, or, if it doesn't exist, from the translation files,
    /// and caches it.
    ///
    /// While it is technically possible to use this function directly, it is not recommended.
    /// Consider using the `tr!` or `trn!` macros instead, and, if functionality is missing from
    /// the macros, file an issue.
    pub fn lookup_cached<'a>(
        &self,
        key: &str,
        variables: &'a [Option<LookupVariable<'a>>],
        lookup_crate: &str,
        hash: u64,
        locale_override: Option<&Locale>,
    ) -> I18nString {
        let mut state = FxHasher::default();
        hash.hash(&mut state);
        for variable in variables.into_iter().flatten() {
            variable.1.hash_value(&mut state);
        }
        if let Some(locale) = locale_override {
            (&locale).hash(&mut state);
        }
        let full_call_hash = state.finish();

        self.cache.get(&full_call_hash).clone().unwrap_or_else(|| {
            let result = self.lookup(key, variables, lookup_crate, locale_override);
            self.cache.insert(full_call_hash, result.clone());
            result
        })
    }

    /// Lookup a translation from the translation files.
    ///
    /// While it is technically possible to use this function directly, it is not recommended.
    /// Consider using the `tr!` or `trn!` macros instead, and, if functionality is missing from
    /// the macros, file an issue.
    pub fn lookup<'a>(
        &self,
        key: &str,
        variables: &'a [Option<LookupVariable<'a>>],
        lookup_crate: &str,
        locale_override: Option<&Locale>,
    ) -> I18nString {
        let locale = locale_override.unwrap_or(&self.locale);

        for source in &self.sources {
            let Some(entry) = source.lookup(locale, key, lookup_crate) else {
                continue;
            };

            // TODO: Cache the resolved string
            let resolved = I18nString::Owned(
                match &entry {
                    I18nEntry::Entry(entry) => entry.to_vec(),
                    I18nEntry::PluralEntry(entry) => {
                        let (_, count) = variables
                            .into_iter()
                            .find(|variable| match variable {
                                Some((name, _)) => *name == "count",
                                None => false,
                            })
                            .map(Option::as_deref)
                            .flatten()
                            .unwrap_or_else(|| {
                                panic!(
                                    "Resolved plural string for {key}, but no count variable \
                                    provided for substitution",
                                )
                            });

                        match count {
                            Variable::Count(count) => entry.lookup(*count, locale),
                            Variable::String(string) => {
                                panic!("Count variable ({string}) not of type isize")
                            }
                            Variable::Modified(_inital, _subsequent) => {
                                panic!("Cannot modify count variable")
                            }
                        }
                    }
                }
                .iter()
                .map(|part| match part {
                    I18nStringPart::Static(borrowed) => borrowed.to_string(),
                    I18nStringPart::Variable(variable, idx) => {
                        let substituted_variable = variables
                            .get(*idx)
                            .map(Option::as_deref)
                            .flatten()
                            .filter(|(name, _)| *name == variable.as_ref())
                            .or_else(|| {
                                // Fallback for if idx is out of bounds or doesn't match the variable name
                                variables
                                    .into_iter()
                                    .map(Option::as_deref)
                                    .flatten()
                                    .find(|(name, _)| *name == variable.as_ref())
                            })
                            .map(|(_, variable)| variable);

                        match substituted_variable {
                            Some(Variable::Modified(initial, subsequent)) => subsequent
                                .iter()
                                .fold(initial.transform(locale), |v, modi| {
                                    modi.0.transform(locale, v, modi.1)
                                }),
                            Some(Variable::String(str)) => str.into(),
                            Some(Variable::Count(_)) => {
                                panic!("Unexpected count variable")
                            }
                            None => format!("{{{{{variable}}}}}"),
                        }
                    }
                    I18nStringPart::Count(_) => "{{count}}".to_string(),
                })
                .collect::<Vec<_>>()
                .join("")
                .into(),
            );

            // If the translation is empty, fall back to the next source
            if resolved.is_empty() {
                continue;
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
            cache: Cache::new(500),
        }
    }
}
