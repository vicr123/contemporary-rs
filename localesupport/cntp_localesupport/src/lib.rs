//! # `cntp_localesupport`
//!
//! This crate provides locale-aware formatting and string modification utilities for the
//! `cntp_i18n` system. It handles locale detection, number formatting, date/time formatting,
//! text direction, and string modifiers.
//!
//! ## Overview
//!
//! The main type in this crate is [`Locale`], which represents the user's locale settings
//! for messages, numeric formatting, and time formatting. You can obtain the current system
//! locale or create one from a language identifier:
//!
//! ```rust
//! use cntp_localesupport::Locale;
//!
//! // Get the current system locale
//! let locale = Locale::current();
//!
//! // Or create from a specific identifier
//! let locale = Locale::new_from_locale_identifier("en-US");
//! ```
//!
//! ## Layout direction
//!
//! The crate provides support for determining text layout direction (left-to-right or
//! right-to-left) based on the locale:
//!
//! ```rust
//! use cntp_localesupport::{Locale, LayoutDirection};
//!
//! let locale = Locale::new_from_locale_identifier("ar");
//! match locale.layout_direction() {
//!     LayoutDirection::RightToLeft => println!("RTL layout"),
//!     LayoutDirection::LeftToRight => println!("LTR layout"),
//! }
//! ```
//!
//! ## String modifiers
//!
//! The [`modifiers`] module provides string transformation utilities that respect locale
//! conventions. These are typically used with the `tr!` macro from `cntp_i18n`, but can
//! also be used directly:
//!
//! - [`modifiers::Date`] - Format dates and times according to locale conventions
//! - [`modifiers::Quote`] - Wrap strings in locale-appropriate quotation marks
//!
//! ## Locale-aware formatting
//!
//! The [`locale_formattable::LocaleFormattable`] trait allows values to be formatted
//! according to locale conventions. This is implemented for numeric types and strings:
//!
//! ```rust
//! use cntp_localesupport::Locale;
//! use cntp_localesupport::locale_formattable::LocaleFormattable;
//!
//! let locale = Locale::new_from_locale_identifier("de-DE");
//! let number: i32 = 1234567;
//! // In the German locale, this would format as "1.234.567"
//! let formatted = number.to_locale_string(&locale);
//! ```
//!
//! ## Feature flags
//!
//! - `chrono` (default): Enables support for formatting `chrono` date/time types with the
//!   [`modifiers::Date`] modifier.

#![warn(missing_docs)]

mod cldr;
pub mod locale_formattable;
pub mod modifiers;

use crate::cldr::CldrData;
use icu::decimal::DecimalFormatter;
use icu::decimal::input::Decimal;
use icu::locale::subtags::{Language, Region};
use icu::locale::{Direction, Locale as IcuLocale, LocaleDirectionality, locale};
use locale_config::Locale as LocaleConfigLocale;

use smallvec::SmallVec;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

/// Represents the text layout direction for a locale.
///
/// This is used to determine whether text should flow left-to-right (like English)
/// or right-to-left (like Arabic or Hebrew).
///
/// # Example
///
/// ```rust
/// use cntp_localesupport::{Locale, LayoutDirection};
///
/// let english = Locale::new_from_locale_identifier("en");
/// assert!(matches!(english.layout_direction(), LayoutDirection::LeftToRight));
///
/// let arabic = Locale::new_from_locale_identifier("ar");
/// assert!(matches!(arabic.layout_direction(), LayoutDirection::RightToLeft));
/// ```
pub enum LayoutDirection {
    /// Text flows from left to right (e.g., English, French, German).
    LeftToRight,
    /// Text flows from right to left (e.g., Arabic, Hebrew, Persian).
    RightToLeft,
}

/// Represents locale settings for internationalization.
///
/// A `Locale` contains settings for three different categories:
/// - **Messages**: Used for translated text lookups
/// - **Numeric**: Used for number formatting (decimal separators, grouping, etc.)
/// - **Time**: Used for date and time formatting
///
/// Each category can have a fallback chain of locales. For example, a user might
/// prefer `en-GB` for messages but `en-US` for numeric formatting.
///
/// # Creating a Locale
///
/// ```rust
/// use cntp_localesupport::Locale;
///
/// // Get the current system locale (recommended for most applications)
/// let system_locale = Locale::current();
///
/// // Create from a single identifier (uses same locale for all categories)
/// let locale = Locale::new_from_locale_identifier("fr-CA");
///
/// // Create with different locales for each category
/// let locale = Locale::new_from_parts(
///     vec!["en-GB".to_string(), "en".to_string()],  // messages
///     vec!["en-US".to_string()],                     // numeric
///     vec!["en-GB".to_string()],                     // time
/// );
/// ```
///
/// # Locale Names
///
/// You can get human-readable names for locales:
///
/// ```rust
/// use cntp_localesupport::Locale;
///
/// let french = Locale::new_from_locale_identifier("fr-FR");
/// // Returns "French (France)" when displayed in English
/// let english = Locale::new_from_locale_identifier("en");
/// println!("{}", french.human_readable_locale_name_in(&english));
/// ```
pub struct Locale {
    /// The message locale fallback chain (e.g., `["en-US", "en"]`).
    pub messages: Vec<String>,
    /// The numeric formatting locale fallback chain.
    pub numeric: Vec<String>,
    /// The time formatting locale fallback chain.
    pub time: Vec<String>,
    messages_icu: IcuLocale,
    numeric_icu: IcuLocale,
    time_icu: IcuLocale,
    cldr_data: HashMap<String, CldrData>,
}

impl Hash for Locale {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.messages_icu.hash(state);
        self.numeric_icu.hash(state);
        self.time_icu.hash(state);
    }
}

/// Errors that can occur when working with locales.
#[derive(Debug)]
pub enum LocaleError {
    /// The locale does not have a region component (e.g., "en" instead of "en-US").
    RegionAgnosticError,
    /// The locale string could not be parsed as a valid locale.
    CustomLocaleError,
}

impl Locale {
    fn split_language_range(language_range: &str) -> SmallVec<[String; 4]> {
        let mut result = SmallVec::new();
        let segments: SmallVec<[&str; 4]> = language_range.split('-').collect();

        for i in (1..=segments.len()).rev() {
            result.push(segments[..i].join("-"));
        }

        result
    }

    fn create_icu_locale(range: &str) -> Option<icu::locale::Locale> {
        for range in Self::split_language_range(range) {
            if let Ok(locale) = icu::locale::Locale::try_from_str(&range) {
                return Some(locale);
            }
        }
        None
    }

    /// Creates a new `Locale` with separate locale chains for each category.
    ///
    /// Each parameter is a fallback chain of locale identifiers. The first locale
    /// in each chain is the preferred one, with subsequent entries serving as fallbacks.
    ///
    /// # Arguments
    ///
    /// * `messages` - Locale chain for message translations
    /// * `numeric` - Locale chain for number formatting
    /// * `time` - Locale chain for date/time formatting
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    ///
    /// let locale = Locale::new_from_parts(
    ///     vec!["pt-BR".to_string(), "pt".to_string()],
    ///     vec!["pt-BR".to_string()],
    ///     vec!["pt-BR".to_string()],
    /// );
    /// ```
    pub fn new_from_parts(
        messages: Vec<String>,
        numeric: Vec<String>,
        time: Vec<String>,
    ) -> Locale {
        // When we add other lookup areas to this locale we should collect them all here and
        // dedupe the array
        let required_cldr_data = messages
            .clone()
            .into_iter()
            .chain(numeric.clone())
            .chain(time.clone())
            .collect::<HashSet<_>>();

        Locale {
            messages_icu: Self::create_icu_locale(messages.first().unwrap())
                .unwrap_or_else(|| Self::create_icu_locale("en").unwrap()),
            messages,
            numeric_icu: Self::create_icu_locale(numeric.first().unwrap())
                .unwrap_or_else(|| Self::create_icu_locale("en").unwrap()),
            numeric,
            time_icu: Self::create_icu_locale(time.first().unwrap())
                .unwrap_or_else(|| Self::create_icu_locale("en").unwrap()),
            time,
            cldr_data: required_cldr_data
                .into_iter()
                .map(|language| {
                    let cldr_data = CldrData::new(language.as_str());
                    (language, cldr_data)
                })
                .collect(),
        }
    }

    /// Creates a new `Locale` from a `locale_config::Locale`.
    ///
    /// This extracts the appropriate locale tags for messages, numeric, and time
    /// categories from the platform's locale configuration.
    pub fn new_from_locale_config_locale(locale_config_locale: LocaleConfigLocale) -> Locale {
        let extract_language_range = |tag: &str| -> Vec<String> {
            let mut parts = locale_config_locale
                .tags_for(tag)
                .flat_map(|language_range| Locale::split_language_range(language_range.as_ref()))
                .filter(|language_range| !language_range.is_empty())
                .peekable();

            if parts.peek().is_none() {
                vec!["en".to_string()]
            } else {
                parts.collect()
            }
        };

        Self::new_from_parts(
            extract_language_range("messages"),
            extract_language_range("numeric"),
            extract_language_range("time"),
        )
    }

    /// Creates a new `Locale` from a single locale identifier.
    ///
    /// The same identifier will be used for messages, numeric, and time formatting.
    ///
    /// # Arguments
    ///
    /// * `identifier` - A locale identifier string (e.g., "en-US", "fr", "zh-Hant-TW")
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    ///
    /// let locale = Locale::new_from_locale_identifier("ja-JP");
    /// ```
    pub fn new_from_locale_identifier(identifier: impl Into<String>) -> Locale {
        let identifier = identifier.into();
        Self::new_from_parts(
            vec![identifier.clone()],
            vec![identifier.clone()],
            vec![identifier],
        )
    }

    /// Returns `true` if this locale has a region component.
    ///
    /// For example, "en-US" has a region (US) while "en" does not.
    pub fn is_regional(&self) -> bool {
        self.messages_icu.id.region.is_some()
    }

    /// Gets the current system locale.
    ///
    /// This reads the locale settings from the operating system and creates
    /// a `Locale` with appropriate fallback chains for each category.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    ///
    /// let locale = Locale::current();
    /// println!("System locale: {}", locale.messages.first().unwrap());
    /// ```
    pub fn current() -> Locale {
        Self::new_from_locale_config_locale(LocaleConfigLocale::current())
    }

    /// Returns the human-readable name of this locale in its own language.
    ///
    /// For example, for French this would return "français (France)".
    pub fn human_readable_locale_name(&self) -> String {
        Self::human_readable_locale_name_internal(self, self)
    }

    /// Returns the human-readable name of this locale in another locale's language.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    ///
    /// let french = Locale::new_from_locale_identifier("fr-FR");
    /// let english = Locale::new_from_locale_identifier("en");
    /// // Returns "French (France)"
    /// let name = french.human_readable_locale_name_in(&english);
    /// ```
    pub fn human_readable_locale_name_in(&self, other: &Locale) -> String {
        Self::human_readable_locale_name_internal(self, other)
    }

    /// Returns the human-readable name of another locale in this locale's language.
    ///
    /// This is the inverse of [`human_readable_locale_name_in`](Self::human_readable_locale_name_in).
    pub fn human_readable_locale_name_of(&self, other: &Locale) -> String {
        Self::human_readable_locale_name_internal(other, self)
    }

    fn human_readable_locale_name_internal(of: &Locale, r#in: &Locale) -> String {
        let language = r#in.human_readable_language_name_of(of);
        let region = r#in.human_readable_region_name_of(of);
        let Some(region) = region else {
            return language;
        };
        format!("{language} ({region})")
    }

    /// Returns the human-readable language name (without region) in its own language.
    ///
    /// For example, for "fr-CA" this would return "français".
    pub fn human_readable_language_name(&self) -> String {
        self.human_readable_language_name_in(self)
    }

    /// Returns the human-readable language name in another locale's language.
    pub fn human_readable_language_name_in(&self, other: &Locale) -> String {
        Self::human_readable_language_name_internal(self, other)
    }

    /// Returns the human-readable language name of another locale in this locale's language.
    pub fn human_readable_language_name_of(&self, other: &Locale) -> String {
        Self::human_readable_language_name_internal(other, self)
    }

    fn human_readable_language_name_internal(of: &Locale, r#in: &Locale) -> String {
        let locale = &r#in.messages_icu;
        let Ok(display_names) = icu::experimental::displaynames::LanguageDisplayNames::try_new(
            locale.clone().into(),
            Default::default(),
        ) else {
            return "Unknown Language".into();
        };

        let Ok(language) = of.try_into() else {
            return "Unknown Language".into();
        };

        display_names.of(language).unwrap_or("").into()
    }

    /// Returns the human-readable region name in its own language, if available.
    ///
    /// Returns `None` if the locale has no region component.
    pub fn human_readable_region_name(&self) -> Option<String> {
        self.human_readable_region_name_in(self)
    }

    /// Returns the human-readable region name in another locale's language.
    ///
    /// Returns `None` if this locale has no region component.
    pub fn human_readable_region_name_in(&self, other: &Locale) -> Option<String> {
        Self::human_readable_region_name_internal(self, other)
    }

    /// Returns the human-readable region name of another locale in this locale's language.
    ///
    /// Returns `None` if the other locale has no region component.
    pub fn human_readable_region_name_of(&self, other: &Locale) -> Option<String> {
        Self::human_readable_region_name_internal(other, self)
    }

    fn human_readable_region_name_internal(of: &Locale, r#in: &Locale) -> Option<String> {
        let locale = &r#in.messages_icu;
        let Ok(display_names) = icu::experimental::displaynames::RegionDisplayNames::try_new(
            locale.clone().into(),
            Default::default(),
        ) else {
            return None;
        };

        let region = of.try_into();
        let Ok(region) = region else {
            return None;
        };

        display_names
            .of(region)
            .map(|region_name| region_name.to_string())
    }

    /// Wraps a string in the locale's primary quotation marks.
    ///
    /// Different locales use different quotation marks. For example:
    /// - English: "Hello"
    /// - French: « Hello »
    /// - German: „Hello"
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    ///
    /// let locale = Locale::new_from_locale_identifier("en-US");
    /// let quoted = locale.quote_string("Hello");
    /// // Returns "\"Hello\""
    /// ```
    pub fn quote_string(&self, string: impl Display) -> String {
        let cldr_locale = self.messages.first().unwrap();
        let delimiters = &self
            .cldr_data
            .get(cldr_locale)
            .expect("CLDR data for messages locale not created.")
            .delimiters;
        format!(
            "{}{string}{}",
            delimiters.quotation_start, delimiters.quotation_end
        )
    }

    /// Wraps a string in the locale's alternate (inner) quotation marks.
    ///
    /// These are typically used for quotes within quotes. For example:
    /// - English: 'Hello'
    /// - French: ‹ Hello ›
    pub fn quote_string_alternate(&self, string: impl Display) -> String {
        let cldr_locale = self.messages.first().unwrap();
        let delimiters = &self
            .cldr_data
            .get(cldr_locale)
            .expect("CLDR data for messages locale not created.")
            .delimiters;
        format!(
            "{}{string}{}",
            delimiters.alternate_quotation_start, delimiters.alternate_quotation_end
        )
    }

    fn create_decimal_formatter(&self) -> DecimalFormatter {
        DecimalFormatter::try_new(self.numeric_icu.clone().into(), Default::default())
            .unwrap_or_else(|_| {
                DecimalFormatter::try_new(locale!("en").into(), Default::default()).unwrap()
            })
    }

    /// Formats a decimal number according to the locale's conventions.
    ///
    /// This applies locale-specific formatting including:
    /// - Decimal separators (e.g., "." vs ",")
    /// - Thousands grouping (e.g., "1,234,567" vs "1.234.567")
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::Locale;
    /// use icu::decimal::input::Decimal;
    ///
    /// let german = Locale::new_from_locale_identifier("de-DE");
    /// let formatted = german.format_decimal(Decimal::from(1234567));
    /// // Returns "1.234.567"
    /// ```
    pub fn format_decimal<T>(&self, i: T) -> String
    where
        T: Into<Decimal>,
    {
        let d = i.into();
        self.create_decimal_formatter().format_to_string(&d)
    }

    /// Returns the text layout direction for this locale.
    ///
    /// # Example
    ///
    /// ```rust
    /// use cntp_localesupport::{Locale, LayoutDirection};
    ///
    /// let locale = Locale::new_from_locale_identifier("he");
    /// match locale.layout_direction() {
    ///     LayoutDirection::RightToLeft => println!("Hebrew is RTL"),
    ///     LayoutDirection::LeftToRight => println!("LTR"),
    /// }
    /// ```
    pub fn layout_direction(&self) -> LayoutDirection {
        let directionality = LocaleDirectionality::new_common();

        match directionality
            .get(&self.messages_icu.id)
            .unwrap_or(Direction::LeftToRight)
        {
            Direction::LeftToRight => LayoutDirection::LeftToRight,
            Direction::RightToLeft => LayoutDirection::RightToLeft,
            _ => LayoutDirection::LeftToRight,
        }
    }
}

impl TryFrom<&Locale> for Language {
    type Error = LocaleError;

    fn try_from(value: &Locale) -> Result<Self, Self::Error> {
        let message_language = value.messages.first().unwrap();
        let Ok(locale) = icu::locale::Locale::try_from_str(message_language) else {
            return Err(LocaleError::CustomLocaleError);
        };
        Ok(locale.id.language)
    }
}

impl TryFrom<&Locale> for Region {
    type Error = LocaleError;

    fn try_from(value: &Locale) -> Result<Self, Self::Error> {
        let message_language = value.messages.first().unwrap();
        let Ok(locale) = icu::locale::Locale::try_from_str(message_language) else {
            return Err(LocaleError::CustomLocaleError);
        };
        locale.id.region.ok_or(LocaleError::RegionAgnosticError)
    }
}
