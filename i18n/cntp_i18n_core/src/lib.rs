//! # `cntp-i18n-core`
//!
//! This crate provides the core types and traits for the `cntp-i18n` system.
//! It is primarily used internally by `cntp_i18n` and `cntp_i18n_macros`, but is
//! also useful for advanced use cases like implementing custom translation sources.
//!
//! ## Overview
//!
//! Most users should use the `cntp_i18n` crate directly, which re-exports all
//! necessary types from this crate. This crate is only needed if you want to:
//!
//! - Implement a custom [`I18nSource`] for loading translations from a non-standard location
//! - Work with translation entries directly without going through the macro system
//! - Build tooling that processes translation data
//!
//! ## Core Types
//!
//! - [`I18nSource`] - Trait for translation providers (implement this for custom sources)
//! - [`I18nEntry`] - A translation entry (either singular or plural)
//! - [`I18nStringEntry`] - A simple string translation
//! - [`I18nPluralStringEntry`] - A pluralized translation with multiple forms
//! - [`I18nStringPart`] - A component of a translation (static text, variable, or count)
//! - [`I18nString`](string::I18nString) - An efficient string type (borrowed or owned)
//!
//! ## Feature flags
//!
//! - **`gpui`** - Enables conversion of [`I18nString`](string::I18nString) to GPUI's
//!   `SharedString` for seamless GPUI integration.
//!
//! ## Implementing a Custom Translation Source
//!
//! ```rust,ignore
//! use cntp_i18n_core::{I18nSource, I18nEntry, I18nStringPart};
//! use cntp_localesupport::Locale;
//!
//! struct MyCustomSource {
//!     // ... your translation data
//! }
//!
//! impl I18nSource for MyCustomSource {
//!     fn lookup<'a>(
//!         &'a self,
//!         locale: &Locale,
//!         id: &str,
//!         lookup_crate: &str,
//!     ) -> Option<&'a I18nEntry<'a>> {
//!         // Look up the translation by id and return it
//!         // Return None if not found
//!         None
//!     }
//! }
//! ```

#![warn(missing_docs)]

#[cfg(feature = "gpui")]
/// GPUI integration for `I18nString`.
///
/// This module provides conversions between [`I18nString`](string::I18nString) and
/// GPUI's `SharedString` type for seamless integration with GPUI applications.
///
/// This module is only available when the `gpui` feature is enabled.
pub mod gpui;
/// String types for internationalized text.
///
/// See [`I18nString`](string::I18nString) for the main type.
pub mod string;

use crate::string::I18nString;
use anyhow::anyhow;
use cntp_localesupport::Locale;
use cntp_localesupport::locale_formattable::LocaleFormattable;
use icu::plurals::{PluralCategory, PluralRules};

/// A source of translation data.
///
/// Implement this trait to provide translations from a custom source (database,
/// remote API, custom file format, etc.). The default implementation provided
/// by [`tr_load!`](cntp_i18n_macros::tr_load) loads translations from JSON files
/// at compile time.
///
/// # Thread safety
///
/// Implementations must be `Send + Sync` since they may be accessed from multiple
/// threads concurrently through the global [`I18nManager`](cntp_i18n::I18nManager).
///
/// # Example
///
/// ```rust,ignore
/// use cntp_i18n_core::{I18nSource, I18nEntry};
/// use cntp_localesupport::Locale;
///
/// struct DatabaseTranslationSource {
///     // connection pool, cached data, etc.
/// }
///
/// impl I18nSource for DatabaseTranslationSource {
///     fn lookup<'a>(
///         &'a self,
///         locale: &Locale,
///         id: &str,
///         lookup_crate: &str,
///     ) -> Option<&'a I18nEntry<'a>> {
///         // Query database for translation
///         // The lookup_crate parameter can be used to namespace translations
///         None
///     }
/// }
/// ```
pub trait I18nSource: Send + Sync {
    /// Look up a translation entry by its identifier.
    ///
    /// # Arguments
    ///
    /// * `locale` - The locale to look up the translation for
    /// * `id` - The translation key (e.g., "HELLO_WORLD")
    /// * `lookup_crate` - The crate that requested the translation (for namespacing)
    ///
    /// # Returns
    ///
    /// `Some(&I18nEntry)` if a translation was found, `None` otherwise.
    /// When `None` is returned, the manager will try the next source in the chain.
    fn lookup(&'_ self, locale: &Locale, id: &str, lookup_crate: &str)
    -> Option<&'_ I18nEntry<'_>>;
}

/// A simple (non-plural) translation entry.
///
/// This type wraps a single translated string. It is used for translations
/// that don't vary based on count.
pub struct I18nStringEntry {
    /// The translated string content.
    pub entry: I18nString,
}

/// A pluralized translation entry with multiple forms.
///
/// Different languages have different plural rules. For example:
/// - English has 2 forms: "one" (1 item) and "other" (0, 2+ items)
/// - Arabic has 6 forms: zero, one, two, few, many, other
/// - Japanese has 1 form: "other" (no plural distinction)
///
/// This struct holds all the plural forms for a translation. The appropriate
/// form is selected at runtime based on the count and the locale's plural rules.
///
/// # Required forms
///
/// The `other` form is always required and serves as the fallback. Other forms
/// are optional and depend on the language's plural rules.
///
/// # Example JSON
///
/// ```json
/// {
///     "ITEMS_COUNT": {
///         "one": "{{count}} item",
///         "other": "{{count}} items"
///     }
/// }
/// ```
pub struct I18nPluralStringEntry<'a> {
    /// The locale identifier for plural rule selection.
    pub locale: I18nString,
    /// Translation for the "zero" plural category (e.g., Arabic).
    pub zero: Option<&'a [I18nStringPart]>,
    /// Translation for the "one" plural category (e.g., English singular).
    pub one: Option<&'a [I18nStringPart]>,
    /// Translation for the "two" plural category (e.g., Arabic dual).
    pub two: Option<&'a [I18nStringPart]>,
    /// Translation for the "few" plural category (e.g., Russian 2-4).
    pub few: Option<&'a [I18nStringPart]>,
    /// Translation for the "many" plural category (e.g., Russian 5-20).
    pub many: Option<&'a [I18nStringPart]>,
    /// Translation for the "other" plural category (always required, used as fallback).
    pub other: &'a [I18nStringPart],
}

impl I18nPluralStringEntry<'_> {
    /// Select the appropriate plural form based on the count and locale.
    ///
    /// This method uses ICU plural rules to determine which form to use
    /// for the given count in the specified locale.
    ///
    /// # Arguments
    ///
    /// * `count` - The count value to base plural selection on
    /// * `cntp_locale` - The locale for number formatting in the result
    ///
    /// # Returns
    ///
    /// A vector of string parts with `{{count}}` placeholders replaced
    /// with the locale-formatted count value.
    pub fn lookup(&self, count: isize, cntp_locale: &Locale) -> Vec<I18nStringPart> {
        let lookup_core = || -> anyhow::Result<Vec<I18nStringPart>> {
            let locale = icu::locale::Locale::try_from_str(&self.locale)?;
            let pr = PluralRules::try_new(locale.into(), Default::default())?;

            Ok(match pr.category_for(count) {
                PluralCategory::Zero => self
                    .zero
                    .as_ref()
                    .ok_or(anyhow!("Zero case required but not present"))?,
                PluralCategory::One => self
                    .one
                    .as_ref()
                    .ok_or(anyhow!("One case required but not present"))?,
                PluralCategory::Two => self
                    .two
                    .as_ref()
                    .ok_or(anyhow!("Two case required but not present"))?,
                PluralCategory::Few => self
                    .few
                    .as_ref()
                    .ok_or(anyhow!("Few case required but not present"))?,
                PluralCategory::Many => self
                    .many
                    .as_ref()
                    .ok_or(anyhow!("Many case required but not present"))?,
                PluralCategory::Other => &self.other,
            }
            .iter()
            .map(|part| match part {
                I18nStringPart::Count(_) => {
                    I18nStringPart::Static(count.to_locale_string(cntp_locale).into())
                }
                _ => part.clone(),
            })
            .collect())
        };

        lookup_core().unwrap_or_else(|_| self.other.to_vec())
    }
}

/// A translation entry, which can be either singular or plural.
///
/// This enum represents a single translation unit loaded from a translation file.
/// The macro system uses this to determine how to render the translation.
pub enum I18nEntry<'a> {
    /// A simple, non-plural translation.
    ///
    /// The slice contains the parts that make up the translated string,
    /// including static text and variable placeholders.
    Entry(&'a [I18nStringPart]),
    /// A plural translation with multiple forms based on count.
    PluralEntry(I18nPluralStringEntry<'a>),
}

/// A component of a translated string.
///
/// Translated strings are parsed into parts at compile time. This allows
/// efficient variable substitution at runtime without re-parsing the string.
///
/// # Example
///
/// The translation `"Hello, {{name}}! You have {{count}} messages."` would be
/// parsed into:
///
/// ```text
/// [
///     Static("Hello, "),
///     Variable("name", 0),
///     Static("! You have "),
///     Count(1),
///     Static(" messages."),
/// ]
/// ```
#[derive(Clone)]
pub enum I18nStringPart {
    /// Static text that is output as-is.
    Static(I18nString),
    /// A variable placeholder to be substituted at runtime.
    ///
    /// The first field is the variable name, the second is a hint index
    /// for faster lookup in the variables array.
    Variable(I18nString, usize),
    /// A count placeholder for plural strings.
    ///
    /// The field is a hint index for the count variable.
    Count(usize),
}

impl I18nEntry<'_> {
    /// Returns `true` if this is a simple (non-plural) entry.
    pub fn is_singular(&self) -> bool {
        match self {
            I18nEntry::Entry(_) => true,
            I18nEntry::PluralEntry(_) => false,
        }
    }

    /// Returns `true` if this is a plural entry.
    pub fn is_plural(&self) -> bool {
        !self.is_singular()
    }
}
