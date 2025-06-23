pub mod config;
#[cfg(feature = "gpui")]
pub mod gpui;
pub mod load;
pub mod string;

use anyhow::anyhow;
use contemporary_localesupport::Locale;
use icu::plurals::{PluralCategory, PluralRules};

use crate::string::I18nString;

pub trait I18nSource: Send + Sync {
    fn lookup(&self, locale: &Locale, id: &str) -> Option<&I18nEntry>;
}

pub struct I18nStringEntry {
    pub entry: I18nString,
}

pub struct I18nPluralStringEntry {
    pub locale: I18nString,
    pub zero: Option<I18nString>,
    pub one: Option<I18nString>,
    pub two: Option<I18nString>,
    pub few: Option<I18nString>,
    pub many: Option<I18nString>,
    pub other: I18nString,
}

impl I18nPluralStringEntry {
    pub fn lookup(&self, count: isize) -> I18nString {
        let lookup_core = || -> anyhow::Result<I18nString> {
            let locale = icu::locale::Locale::try_from_str(&self.locale)?;
            let pr = PluralRules::try_new(locale.into(), Default::default())?;

            Ok(match pr.category_for(count) {
                PluralCategory::Zero => self
                    .zero
                    .as_ref()
                    .ok_or(anyhow!("Zero case required but not present"))?
                    .replace("{{count}}", &*count.to_string())
                    .into(),
                PluralCategory::One => self
                    .one
                    .as_ref()
                    .ok_or(anyhow!("One case required but not present"))?
                    .replace("{{count}}", &*count.to_string())
                    .into(),
                PluralCategory::Two => self
                    .two
                    .as_ref()
                    .ok_or(anyhow!("Two case required but not present"))?
                    .replace("{{count}}", &*count.to_string())
                    .into(),
                PluralCategory::Few => self
                    .few
                    .as_ref()
                    .ok_or(anyhow!("Few case required but not present"))?
                    .replace("{{count}}", &*count.to_string())
                    .into(),
                PluralCategory::Many => self
                    .many
                    .as_ref()
                    .ok_or(anyhow!("Many case required but not present"))?
                    .replace("{{count}}", &*count.to_string())
                    .into(),
                PluralCategory::Other => {
                    self.other.replace("{{count}}", &*count.to_string()).into()
                }
            })
        };

        lookup_core()
            .unwrap_or_else(|_| self.other.replace("{{count}}", &*count.to_string()).into())
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
