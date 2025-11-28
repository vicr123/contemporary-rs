pub mod config;
#[cfg(feature = "gpui")]
pub mod gpui;
pub mod load;
pub mod string;

use crate::string::I18nString;
use anyhow::anyhow;
use cntp_localesupport::Locale;
use cntp_localesupport::locale_formattable::LocaleFormattable;
use icu::plurals::{PluralCategory, PluralRules};

pub trait I18nSource: Send + Sync {
    fn lookup(&'_ self, locale: &Locale, id: &str, lookup_crate: &str)
    -> Option<&'_ I18nEntry<'_>>;
}

pub struct I18nStringEntry {
    pub entry: I18nString,
}

pub struct I18nPluralStringEntry<'a> {
    pub locale: I18nString,
    pub zero: Option<&'a [I18nStringPart]>,
    pub one: Option<&'a [I18nStringPart]>,
    pub two: Option<&'a [I18nStringPart]>,
    pub few: Option<&'a [I18nStringPart]>,
    pub many: Option<&'a [I18nStringPart]>,
    pub other: &'a [I18nStringPart],
}

impl I18nPluralStringEntry<'_> {
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
            .into_iter()
            .map(|part| match part {
                I18nStringPart::Count(_) => {
                    I18nStringPart::Static(count.to_locale_string(&cntp_locale).into())
                }
                _ => part.clone(),
            })
            .collect())
        };

        lookup_core().unwrap_or_else(|_| self.other.to_vec())
    }
}

pub enum I18nEntry<'a> {
    Entry(&'a [I18nStringPart]),
    PluralEntry(I18nPluralStringEntry<'a>),
}

#[derive(Clone)]
pub enum I18nStringPart {
    Static(I18nString),
    Variable(I18nString, usize),
    Count(usize),
}

impl I18nEntry<'_> {
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
