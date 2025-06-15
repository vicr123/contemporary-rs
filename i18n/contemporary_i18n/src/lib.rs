pub use contemporary_i18n_macros::{tr, trn};

#[cfg(feature = "gpui")]
use gpui::Global;

use contemporary_i18n_core::I18nSource;

use locale_config::Locale;

pub struct I18nManager {
    sources: Vec<Box<dyn I18nSource>>,
    locale: Locale,
}

impl I18nManager {
    pub fn new() -> I18nManager {
        I18nManager {
            sources: vec![],
            locale: Locale::current(),
        }
    }

    pub fn lookup<'a>(&self, key: &'a str) -> &'a str {
        key
    }
}

#[cfg(feature = "gpui")]
impl Global for I18nManager {}
