use cntp_i18n_core::string::I18nString;
use cntp_i18n_core::{I18nEntry, I18nSource, I18nStringPart};
use cntp_localesupport::Locale;

pub struct HardcodedI18nSource;

impl I18nSource for HardcodedI18nSource {
    fn lookup(&'_ self, _: &Locale, id: &str, _: &str) -> Option<&'_ I18nEntry<'_>> {
        if id == "TR_F" {
            return Some(&I18nEntry::Entry(&[I18nStringPart::Variable(
                I18nString::Borrowed("variable"),
                0,
            )]));
        }

        None
    }
}
