//! Efficient string type for internationalization.
//!
//! This module provides [`I18nString`], an optimized string type that can be either
//! a borrowed static string (for compile-time translations) or an owned reference-counted
//! string (for runtime-constructed translations).

use std::{
    fmt::{Debug, Display},
    ops::Deref,
    sync::Arc,
};

/// An efficient string type for internationalized text.
///
/// `I18nString` is designed for the i18n use case where strings are often static
/// (embedded at compile time) but sometimes need to be constructed at runtime
/// (e.g., when substituting variables).
///
/// # Variants
///
/// - [`Borrowed`](I18nString::Borrowed) - A reference to a static string, zero-cost to clone
/// - [`Owned`](I18nString::Owned) - A reference-counted string, cheap to clone
///
/// # Usage
///
/// You typically don't construct `I18nString` directly. Instead, you receive it
/// from the [`tr!`](cntp_i18n_macros::tr) and [`trn!`](cntp_i18n_macros::trn) macros.
///
/// ```rust,ignore
/// use cntp_i18n::tr;
///
/// let greeting: I18nString = tr!("HELLO", "Hello!");
///
/// // Use as a string
/// println!("{}", greeting);
///
/// // Get a &str reference
/// let s: &str = &greeting;
///
/// // Convert to String
/// let owned: String = greeting.into();
/// ```
///
/// # Conversions
///
/// `I18nString` implements several conversion traits:
///
/// - `From<&'static str>` - Create a borrowed variant
/// - `From<String>` - Create an owned variant
/// - `From<Arc<str>>` - Create an owned variant
/// - `Into<String>` - Convert to an owned `String`
/// - `Into<Arc<str>>` - Convert to a reference-counted string
/// - `Deref<Target=str>` - Use as a `&str`
/// - `AsRef<str>` - Use as a `&str`
/// - `Display` - Format for display
pub enum I18nString {
    /// A borrowed reference to a static string.
    ///
    /// This variant is used for translations that are fully static (no variable
    /// substitution needed).
    Borrowed(&'static str),
    /// An owned, reference-counted string.
    ///
    /// This variant is used for translations that require runtime construction,
    /// such as those with variable substitution. The `Arc<str>` allows cheap
    /// cloning without full string copies.
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

/// Error returned when trying to convert an owned `I18nString` to a static `&str`.
///
/// This error occurs when calling `TryInto<&'static str>` on an `I18nString::Owned`
/// variant, since owned strings cannot be converted to static references.
pub enum I18nToStrError {
    /// The string was owned, not borrowed, and cannot be converted to `&'static str`.
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

impl From<I18nString> for Arc<str> {
    fn from(val: I18nString) -> Self {
        match val {
            I18nString::Borrowed(s) => Arc::from(s),
            I18nString::Owned(s) => s,
        }
    }
}

impl From<I18nString> for String {
    fn from(val: I18nString) -> Self {
        match val {
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
