pub mod config;
pub mod load;

use locale_config::Locale;

pub trait I18nSource: Send + Sync {
    fn lookup(&self, locale: &Locale, id: &str) -> Option<I18nEntry>;
}

pub struct I18nStringEntry {
    pub entry: String,
}

pub struct I18nPluralStringEntry {
    locale: Locale,
    zero: Option<String>,
    one: Option<String>,
    two: Option<String>,
    few: Option<String>,
    many: Option<String>,
    other: String,
}

impl I18nPluralStringEntry {
    pub fn lookup(&self, count: isize) -> String {
        format!("Looked up plural string with count {}", count)
    }
}

pub enum I18nEntry {
    Entry(I18nStringEntry),
    PluralEntry(I18nPluralStringEntry),
}

impl I18nEntry {
    pub fn is_singular(&self) -> bool {
        match self {
            I18nEntry::Entry(_) => true,
            I18nEntry::PluralEntry(_) => false,
        }
    }

    pub fn is_plural(&self) -> bool {
        !self.is_singular()
    }
}

struct ContemporaryI18nSource;

impl I18nSource for ContemporaryI18nSource {
    fn lookup(&self, locale: &Locale, id: &str) -> Option<I18nEntry> {
        Some(I18nEntry::Entry(I18nStringEntry {
            entry: "Looked up string".to_string(),
        }))
    }
}
