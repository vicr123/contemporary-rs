use icu::locale::Locale as IcuLocale;
use icu::locale::subtags::{Language, Region};
use locale_config::Locale as LocaleConfigLocale;
use std::fmt::Display;

pub struct Locale {
    pub messages: Vec<String>,
    messages_icu: IcuLocale,
}

pub enum LocaleError {
    RegionAgnosticError,
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

    pub fn new_from_parts(messages: Vec<String>) -> Locale {
        Locale {
            messages_icu: Self::create_icu_locale(messages.first().unwrap())
                .unwrap_or_else(|| Self::create_icu_locale("en").unwrap()),
            messages,
        }
    }

    pub fn new_from_locale_config_locale(locale_config_locale: LocaleConfigLocale) -> Locale {
        Self::new_from_parts(
            locale_config_locale
                .tags_for("messages")
                .flat_map(|language_range| Self::split_language_range(language_range.as_ref()))
                .collect(),
        )
    }

    pub fn new_from_locale_identifier(identifier: impl Into<String>) -> Locale {
        Self::new_from_parts(vec![identifier.into()])
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
        let display_names = icu::experimental::displaynames::LanguageDisplayNames::try_new(
            locale.clone().into(),
            Default::default(),
        )
        .unwrap();
        display_names.of(of.into()).unwrap_or("").into()
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
        let display_names = icu::experimental::displaynames::RegionDisplayNames::try_new(
            locale.clone().into(),
            Default::default(),
        )
        .unwrap();
        let region = of.try_into();
        let Ok(region) = region else {
            return None;
        };

        display_names
            .of(region)
            .map(|region_name| region_name.to_string())
    }

    pub fn quote_string(&self, string: impl Display) -> String {
        format!("\"{string}\"")
    }

    pub fn quote_string_alternate(&self, string: impl Display) -> String {
        format!("'{string}'")
    }
}

impl From<&Locale> for Language {
    fn from(value: &Locale) -> Self {
        let message_language = value.messages.first().unwrap();
        let locale = icu::locale::Locale::try_from_str(message_language).unwrap();
        locale.id.language
    }
}

impl TryFrom<&Locale> for Region {
    type Error = LocaleError;

    fn try_from(value: &Locale) -> Result<Self, Self::Error> {
        let message_language = value.messages.first().unwrap();
        let locale = icu::locale::Locale::try_from_str(message_language).unwrap();
        locale.id.region.ok_or(LocaleError::RegionAgnosticError)
    }
}
