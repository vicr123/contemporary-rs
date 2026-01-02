//! # `cntp_i18n`
//!
//! `cntp_i18n` is an ergonomic internationalization (i18n) library for Rust.
//!
//! ## Setup
//!
//! Add `cntp_i18n` to your dependencies:
//!
//! ```toml
//! [dependencies]
//! cntp_i18n = { git = "https://github.com/vicr123/contemporary-rs" }
//! ```
//!
//! ### Loading translations
//!
//! Initialize the translation system at application startup using [`tr_load!`] to load
//! the translations into the binary. This must be done before calling [`tr!`] or [`trn!`].
//!
//! ```rust,ignore
//! use cntp_i18n::{I18N_MANAGER, tr_load};
//!
//! fn main() {
//!     I18N_MANAGER.write().unwrap().load_source(tr_load!());
//!
//!     // Now you can use tr! and trn! macros
//!     println!("{}", tr!("HELLO_WORLD", "Hello World!"));
//! }
//! ```
//!
//! ## Translation files
//! Translation files are stored in the `translations` directory (configurable via `i18n.toml`):
//!
//! - `en.json` - English translations (or your default language)
//! - `meta.json` - Metadata about each translation string
//! - `fr.json`, `de.json`, etc. - Additional language files
//!
//! Translations can be embedded in your application's macros. There are two ways to generate
//! translation files from tr! and trn! macros:
//!
//! ### `cntp_i18n_gen`
//!
//! Add the `cntp_i18n_gen` to your build dependencies:
//!
//! ```toml
//! [build-dependencies]
//! cntp_i18n_gen = { git = "https://github.com/vicr123/contemporary-rs" }
//! ```
//!
//! Then, add the code to generate the files to your build.rs script:
//!
//! ```rust,ignore
//! use std::{env, path::PathBuf};
//!
//! fn main() {
//!     let path: PathBuf = env::var("CARGO_MANIFEST_DIR")
//!         .expect("CARGO_MANIFEST_DIR is not set")
//!         .into();
//!
//!     cntp_i18n_gen::generate_default(&path);
//! }
//! ```
//!
//! ### `cargo-cntp-i18n`
//!
//! Alternatively, you can use the `cargo cntp-i18n generate` command to manually generate
//! translation files instead of using a build script. See cargo-cntp-i18n for more details.
//!
//! ## Usage
//!
//! Use `tr!` for simple, non-plural strings:
//!
//! ```rust,ignore
//! // Basic usage
//! tr!("BUTTON_SIGN_IN", "Sign In");
//!
//! // With variable substitution
//! tr!("GREETING", "Hello, {{name}}!", name = user_name);
//!
//! // With modifiers (e.g., quoting)
//! tr!("QUOTED_NAME", "Your name is {{name}}.", name:quote = user_name);
//! ```
//!
//! ### Plural strings
//!
//! Use `trn!` for strings that vary based on count:
//!
//! ```rust,ignore
//! trn!(
//!     "UNREAD_EMAILS",
//!     "You have {{count}} unread email.",
//!     "You have {{count}} unread emails.",
//!     count = email_count
//! );
//! ```
//!
//! The `count` variable is expected to be a [`isize`].
//!
//! ## Configuration
//!
//! Create an `i18n.toml` file in your project root to customize behavior:
//!
//! ```toml
//! [i18n]
//! default_language = "en"
//! translation_directory = "translations"
//! match_line_endings = true
//! ```
//!
//! ## Feature flags
//!
//! - **`gpui`** - Enables automatic conversion of translation results to GPUI's `SharedString`.
//! This allows the text to be used as an child easily, without any additional function calls.
//! - **`chrono`** - Enables `chrono` date/time type support in the [`Date`] modifier.
//!
//! ## Architecture
//!
//! This crate re-exports types from several internal crates:
//!
//! - Macros ([`tr!`], [`trn!`], [`tr_load!`]) from `cntp_i18n_macros`
//! - Core types ([`I18nSource`], [`I18nEntry`], etc.) from `cntp_i18n_core`
//! - Locale support ([`Locale`], [`LocaleFormattable`], modifiers) from `cntp_localesupport`
//!
//! For build-time generation, use `cntp_i18n_gen` in your `build.rs`.
//!
//! [`Date`]: modifiers::Date

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

/// The global i18n manager instance.
///
/// This is the central point for managing translations in your application. It holds
/// all loaded translation sources and the current locale settings.
///
/// # Usage
///
/// Load translation sources at application startup:
///
/// ```rust,ignore
/// use cntp_i18n::{I18N_MANAGER, tr_load};
///
/// fn main() {
///     // Load translations (typically done once at startup)
///     I18N_MANAGER.write().unwrap().load_source(tr_load!());
///
///     // Change locale if needed
///     I18N_MANAGER.write().unwrap().locale = Locale::new_from_locale_identifier("fr");
/// }
/// ```
///
/// For read-only access (most common), use the [`i18n_manager!`] macro:
///
/// ```rust,ignore
/// let manager = i18n_manager!();
/// let current_locale = &manager.locale;
/// ```
pub static I18N_MANAGER: Lazy<RwLock<I18nManager>> =
    Lazy::new(|| RwLock::new(I18nManager::default()));

/// Convenience macro to get a read lock on the global [`I18N_MANAGER`].
///
/// This is equivalent to `I18N_MANAGER.read().unwrap()` but more concise.
///
/// # Example
///
/// ```rust,ignore
/// use cntp_i18n::i18n_manager;
///
/// let manager = i18n_manager!();
/// println!("Current locale: {:?}", manager.locale.messages);
/// ```
///
/// # Panics
///
/// Panics if the `RwLock` is poisoned (another thread panicked while holding the lock).
#[macro_export]
macro_rules! i18n_manager {
    () => {
        cntp_i18n::I18N_MANAGER.read().unwrap()
    };
}

/// Manages the state of the i18n system in the application.
///
/// The `I18nManager` is responsible for:
/// - Keeping track of all loaded translation sources
/// - Storing the current locale settings
/// - Caching translated strings for performance
/// - Looking up translations from the appropriate source
///
/// # Translation Lookup Order
///
/// When looking up a translation, the manager searches through sources in the order
/// they were loaded. The first source that provides a non-empty translation wins.
/// If no translation is found, the key itself is returned.
pub struct I18nManager {
    sources: Vec<Box<dyn I18nSource>>,
    /// The current locale used for translation lookups.
    ///
    /// This determines which language translations are retrieved in, as well as
    /// locale-specific formatting (numbers, dates, etc.).
    pub locale: Locale,
    cache: Cache<u64, I18nString>,
}

/// Internal trait for type-erased string modifier transformations.
///
/// This trait is used internally by the macro system to handle modifiers
/// without knowing the concrete input type at compile time.
#[doc(hidden)]
pub trait ErasedStringModifierTransform {
    /// Apply the transformation using the given locale.
    fn transform(&self, locale: &Locale) -> String;
    /// Hash the input value for cache key generation.
    fn hash(&self, state: &mut FxHasher);
}

/// Internal type for the first modifier in a chain.
///
/// This is used by the macro expansion and should not be used directly.
#[doc(hidden)]
pub struct BaseStringModifierInvocation<'a, T: ?Sized + Hash>(
    &'a dyn StringModifier<&'a T>,
    &'a [ModifierVariable<'a>],
    &'a T,
);

impl<'a, T: ?Sized + Hash> BaseStringModifierInvocation<'a, T> {
    /// Create a new base modifier invocation.
    #[doc(hidden)]
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

/// Internal type for subsequent modifiers in a chain.
///
/// This is used by the macro expansion and should not be used directly.
#[doc(hidden)]
pub struct SubsequentStringModifierInvocation<'a>(
    &'a dyn StringModifier<String>,
    &'a [ModifierVariable<'a>],
);

impl<'a> SubsequentStringModifierInvocation<'a> {
    /// Create a new subsequent modifier invocation.
    #[doc(hidden)]
    pub fn new(
        modifier: &'a dyn StringModifier<String>,
        variables: &'a [ModifierVariable<'a>],
    ) -> Self {
        SubsequentStringModifierInvocation(modifier, variables)
    }
}

/// Internal representation of a variable passed to translation lookups.
///
/// This is used by the macro expansion and should not be used directly.
#[doc(hidden)]
pub enum Variable<'a> {
    /// A variable with one or more modifiers applied.
    Modified(
        &'a dyn ErasedStringModifierTransform,
        &'a [SubsequentStringModifierInvocation<'a>],
    ),
    /// A plain string variable.
    String(String),
    /// A count variable for plural lookups.
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
    /// Load a translation source into the manager.
    ///
    /// Translation sources are searched in the order they are loaded. This allows
    /// you to override translations by loading a more specific source after a
    /// general one.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use cntp_i18n::{I18N_MANAGER, tr_load};
    ///
    /// // Load the main application translations
    /// I18N_MANAGER.write().unwrap().load_source(tr_load!());
    ///
    /// // Optionally load additional/override translations
    /// // I18N_MANAGER.write().unwrap().load_source(custom_source);
    /// ```
    ///
    /// # Typical Usage
    ///
    /// Most applications will call this once at startup with `tr_load!()`:
    ///
    /// ```rust,ignore
    /// fn main() {
    ///     I18N_MANAGER.write().unwrap().load_source(tr_load!());
    ///     // ... rest of application
    /// }
    /// ```
    pub fn load_source(&mut self, source: impl I18nSource + 'static) {
        self.sources.push(Box::new(source));
    }

    /// Look up a translation with caching.
    ///
    /// This method first checks the cache for a previously resolved translation.
    /// If not found, it performs a full lookup and caches the result.
    ///
    /// # Note
    ///
    /// This is an internal method used by the [`tr!`] and [`trn!`] macros.
    /// You should use those macros instead of calling this directly.
    ///
    /// If you need functionality not provided by the macros, please file an issue.
    #[doc(hidden)]
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

    /// Look up a translation from the loaded sources.
    ///
    /// This method searches through all loaded translation sources to find a match
    /// for the given key. Variable substitution and plural resolution are handled
    /// automatically.
    ///
    /// # Note
    ///
    /// This is an internal method used by the [`tr!`] and [`trn!`] macros.
    /// You should use those macros instead of calling this directly.
    ///
    /// If you need functionality not provided by the macros, please file an issue.
    #[doc(hidden)]
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
