use icu::locale::subtags::Language;
use locale_config::Locale as LocaleConfigLocale;
use std::fmt::Display;

pub struct Locale {
    pub messages: Vec<String>,
}

impl Locale {
    pub fn new_from_locale_config_locale(locale_config_locale: LocaleConfigLocale) -> Locale {
        Locale {
            messages: locale_config_locale
                .tags_for("messages")
                .flat_map(|language_range| {
                    let mut result = Vec::new();
                    let range_string = language_range.to_string();
                    let segments: Vec<&str> = range_string.split('-').collect();

                    for i in (1..=segments.len()).rev() {
                        result.push(segments[..i].join("-"));
                    }

                    result
                })
                .collect(),
        }
    }

    pub fn new_from_locale_identifier(identifier: &str) -> Locale {
        Locale {
            messages: vec![identifier.to_string()],
        }
    }

    pub fn current() -> Locale {
        Self::new_from_locale_config_locale(LocaleConfigLocale::current())
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
        let message_language = r#in.messages.first().unwrap();
        let locale = icu::locale::Locale::try_from_str(message_language).unwrap();
        let display_names = icu::experimental::displaynames::LanguageDisplayNames::try_new(
            locale.clone().into(),
            Default::default(),
        )
        .unwrap();
        display_names.of(of.into()).unwrap_or("").into()
    }

    pub fn quote_string(&self, string: impl Display) -> String {
        format!("\"{}\"", string.to_string())
    }

    pub fn quote_string_alternate(&self, string: impl Display) -> String {
        format!("'{}'", string.to_string())
    }
}

impl From<&Locale> for Language {
    fn from(value: &Locale) -> Self {
        let message_language = value.messages.first().unwrap();
        let locale = icu::locale::Locale::try_from_str(message_language).unwrap();
        locale.id.language
    }
}
