use std::collections::HashMap;

use intl_pluralrules::PluralCategory;
use locale_config::Locale;

pub trait I18nSource {
    fn lookup(&self, id: &str) -> Option<I18nEntry>;
}

struct I18nStringEntry {
    entry: String,
}

struct I18nPluralStringEntry {
    entries: HashMap<PluralCategory, String>,
}

impl I18nPluralStringEntry {
    fn lookup(&self, locale: Locale, count: i64) -> String {
        "Looked up string".to_string()
    }
}

enum I18nEntry {
    Entry(I18nStringEntry),
    PluralEntry(I18nPluralStringEntry),
}

struct ContemporaryI18nSource;

impl I18nSource for ContemporaryI18nSource {
    fn lookup(&self, id: &str) -> Option<I18nEntry> {
        Some(I18nEntry::Entry(I18nStringEntry {
            entry: "Looked up string".to_string(),
        }))
    }
}
