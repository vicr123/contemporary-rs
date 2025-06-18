pub mod config;
pub mod load;

use anyhow::anyhow;
use icu::plurals::{PluralCategory, PluralRules};
use locale_config::Locale;

pub trait I18nSource: Send + Sync {
    fn lookup(&self, locale: &Locale, id: &str) -> Option<I18nEntry>;
}

pub struct I18nStringEntry {
    pub entry: String,
}

pub struct I18nPluralStringEntry {
    locale: String,
    zero: Option<String>,
    one: Option<String>,
    two: Option<String>,
    few: Option<String>,
    many: Option<String>,
    other: String,
}

impl I18nPluralStringEntry {
    pub fn lookup(&self, count: isize) -> String {
        let lookup_core = || -> anyhow::Result<String> {
            let locale = icu::locale::Locale::try_from_str(&*self.locale)?;
            let pr = PluralRules::try_new(locale.into(), Default::default())?;

            Ok(match pr.category_for(count) {
                PluralCategory::Zero => self.zero.as_ref().ok_or(anyhow!("Zero case required but not present"))?.replace("{{count}}", &*count.to_string()),
                PluralCategory::One => self.one.as_ref().ok_or(anyhow!("One case required but not present"))?.replace("{{count}}", &*count.to_string()),
                PluralCategory::Two => self.two.as_ref().ok_or(anyhow!("Two case required but not present"))?.replace("{{count}}", &*count.to_string()),
                PluralCategory::Few => self.few.as_ref().ok_or(anyhow!("Few case required but not present"))?.replace("{{count}}", &*count.to_string()),
                PluralCategory::Many => self.many.as_ref().ok_or(anyhow!("Many case required but not present"))?.replace("{{count}}", &*count.to_string()),
                PluralCategory::Other => self.other.replace("{{count}}", &*count.to_string())
            })
        };
        
        lookup_core().unwrap_or_else(|_| self.other.replace("{{count}}", &*count.to_string()))
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
