pub mod config;
#[cfg(feature = "gpui")]
pub mod gpui;
pub mod load;
pub mod string;

use std::sync::Arc;

use anyhow::anyhow;
use cntp_localesupport::Locale;
use icu::plurals::{PluralCategory, PluralRules};

use crate::string::I18nString;

pub enum StringPart {
    String(I18nString),
    Variable { name: I18nString, idx: usize },
    Count,
}

#[derive(Clone)]
pub enum StringPartList {
    Static(&'static [StringPart]),
    Dynamic(Arc<[StringPart]>),
}

/// We pre-parse the string data ahead of time when strings are loaded by the tr_load! macro.
/// This allows for more efficient string generation regardless of the stored data.
///
/// If the variable names' hash stored here (which is generated from the variables used by all
/// translations) matches the hash provided by the lookup call, then creating the string is done
/// in O(n) (n being the number of Variable parts in the StoredString) by iterating over the parts,
/// mapping them to the correct value by index, and joining the strings.
///
/// If the variables do not match the stored hash, an O(n*m) (where N is the number of Variable
/// parts in the StoredString and M is the number of provided variables) algorithm is used.
///
/// In either case, the variable search is more efficient than the prior technique of using
/// std::str::replace inside of a loop, which has a time complexity of O(n*m*l) (where N is the
/// length of the string, M is the length of the variable name, and L is the number of
/// variables) and a much more complex workload.
#[derive(Clone)]
pub enum StoredString {
    Parts(StringPartList),
    Final(I18nString),
}

pub trait I18nSource: Send + Sync {
    fn lookup(&self, locale: &Locale, id: &str, lookup_crate: &str) -> Option<&I18nEntry>;
}

pub struct I18nStringEntry {
    pub entry: StoredString,
}

pub struct I18nPluralStringEntry {
    pub locale: I18nString,
    pub zero: Option<StoredString>,
    pub one: Option<StoredString>,
    pub two: Option<StoredString>,
    pub few: Option<StoredString>,
    pub many: Option<StoredString>,
    pub other: StoredString,
}

impl I18nPluralStringEntry {
    pub fn lookup(&self, count: isize) -> StoredString {
        let lookup_core = || -> anyhow::Result<StoredString> {
            let locale = icu::locale::Locale::try_from_str(&self.locale)?;
            let pr = PluralRules::try_new(locale.into(), Default::default())?;

            Ok(match pr.category_for(count) {
                PluralCategory::Zero => self
                    .zero
                    .as_ref()
                    .ok_or(anyhow!("Zero case required but not present"))?
                    .clone(),
                PluralCategory::One => self
                    .one
                    .as_ref()
                    .ok_or(anyhow!("One case required but not present"))?
                    .clone(),
                PluralCategory::Two => self
                    .two
                    .as_ref()
                    .ok_or(anyhow!("Two case required but not present"))?
                    .clone(),
                PluralCategory::Few => self
                    .few
                    .as_ref()
                    .ok_or(anyhow!("Few case required but not present"))?
                    .clone(),
                PluralCategory::Many => self
                    .many
                    .as_ref()
                    .ok_or(anyhow!("Many case required but not present"))?
                    .clone(),
                PluralCategory::Other => self.other.clone(),
            })
        };

        lookup_core().unwrap_or_else(|_| self.other.clone())
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
