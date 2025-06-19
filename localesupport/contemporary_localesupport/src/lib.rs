use locale_config::Locale as LocaleConfigLocale;
use std::fmt::Display;

pub struct Locale {
    inner_locale: LocaleConfigLocale,
}

impl Locale {
    pub fn new_from_locale_config_locale(locale_config_locale: LocaleConfigLocale) -> Locale {
        Locale {
            inner_locale: locale_config_locale,
        }
    }

    pub fn current() -> Locale {
        Self::new_from_locale_config_locale(LocaleConfigLocale::current())
    }

    pub fn messages_languages(&self) -> Vec<String> {
        self.inner_locale
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
            .collect()
    }

    pub fn quote_string(&self, string: impl Display) -> String {
        format!("\"{}\"", string.to_string())
    }
}
