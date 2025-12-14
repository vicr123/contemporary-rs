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
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

pub enum LayoutDirection {
    LeftToRight,
    RightToLeft,
}

pub struct Locale {
    pub messages: Vec<String>,
    pub numeric: Vec<String>,
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

#[derive(Debug)]
pub enum LocaleError {
    RegionAgnosticError,
    CustomLocaleError,
}

impl Locale {
    fn split_language_range(language_range: &str) -> Vec<String> {
        let mut result = Vec::new();
        let segments: Vec<&str> = language_range.split('-').collect();

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

    pub fn new_from_locale_identifier(identifier: impl Into<String>) -> Locale {
        let identifier = identifier.into();
        Self::new_from_parts(
            vec![identifier.clone()],
            vec![identifier.clone()],
            vec![identifier],
        )
    }

    pub fn is_regional(&self) -> bool {
        self.messages_icu.id.region.is_some()
    }

    pub fn current() -> Locale {
        Self::new_from_locale_config_locale(LocaleConfigLocale::current())
    }

    pub fn human_readable_locale_name(&self) -> String {
        Self::human_readable_locale_name_internal(self, self)
    }

    pub fn human_readable_locale_name_in(&self, other: &Locale) -> String {
        Self::human_readable_locale_name_internal(self, other)
    }

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

    pub fn human_readable_language_name(&self) -> String {
        self.human_readable_language_name_in(self)
    }

    pub fn human_readable_language_name_in(&self, other: &Locale) -> String {
        Self::human_readable_language_name_internal(self, other)
    }

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

    pub fn human_readable_region_name(&self) -> Option<String> {
        self.human_readable_region_name_in(self)
    }

    pub fn human_readable_region_name_in(&self, other: &Locale) -> Option<String> {
        Self::human_readable_region_name_internal(self, other)
    }

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

    pub fn format_decimal<T>(&self, i: T) -> String
    where
        T: Into<Decimal>,
    {
        let d = i.into();
        self.create_decimal_formatter().format_to_string(&d)
    }

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
