use gpui::{IntoElement, SharedString};

use crate::string::I18nString;

impl From<I18nString> for SharedString {
    fn from(value: I18nString) -> SharedString {
        match value {
            I18nString::Borrowed(s) => SharedString::from(s),
            I18nString::Owned(s) => SharedString::from(s),
        }
    }
}

impl IntoElement for I18nString {
    fn into_element(self) -> Self::Element {
        SharedString::from(self)
    }

    type Element = SharedString;
}
