use std::{
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

pub enum I18nString {
    Borrowed(&'static str),
    Owned(Arc<str>),
}

impl AsRef<str> for I18nString {
    fn as_ref(&self) -> &str {
        match self {
            I18nString::Borrowed(s) => s,
            I18nString::Owned(s) => s.as_ref(),
        }
    }
}

impl Deref for I18nString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            I18nString::Borrowed(s) => s,
            I18nString::Owned(s) => s.as_ref(),
        }
    }
}

impl PartialEq for I18nString {
    fn eq(&self, other: &Self) -> bool {
        let a = self.as_ref();
        let b = other.as_ref();
        a == b
    }
}

impl From<Arc<str>> for I18nString {
    fn from(value: Arc<str>) -> Self {
        I18nString::Owned(value)
    }
}

impl From<String> for I18nString {
    fn from(value: String) -> Self {
        I18nString::Owned(value.into())
    }
}

impl From<&'static str> for I18nString {
    fn from(value: &'static str) -> Self {
        I18nString::Borrowed(value)
    }
}

pub enum I18nToStrError {
    NotBorrowed,
}

impl TryInto<&'static str> for I18nString {
    type Error = I18nToStrError;

    fn try_into(self) -> Result<&'static str, Self::Error> {
        match self {
            I18nString::Borrowed(s) => Ok(s),
            I18nString::Owned(_) => Err(I18nToStrError::NotBorrowed),
        }
    }
}

impl Into<Arc<str>> for I18nString {
    fn into(self) -> Arc<str> {
        match self {
            I18nString::Borrowed(s) => Arc::from(s),
            I18nString::Owned(s) => s,
        }
    }
}

impl Into<String> for I18nString {
    fn into(self) -> String {
        match self {
            I18nString::Borrowed(s) => s.to_string(),
            I18nString::Owned(s) => s.to_string(),
        }
    }
}

impl Debug for I18nString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            I18nString::Borrowed(s) => Debug::fmt(s, f),
            I18nString::Owned(s) => Debug::fmt(s, f),
        }
    }
}

impl Display for I18nString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            I18nString::Borrowed(s) => Display::fmt(s, f),
            I18nString::Owned(s) => Display::fmt(s, f),
        }
    }
}

impl Clone for I18nString {
    fn clone(&self) -> Self {
        match self {
            I18nString::Borrowed(s) => I18nString::Borrowed(s),
            I18nString::Owned(s) => I18nString::Owned(s.clone()),
        }
    }
}
