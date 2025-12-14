#![warn(missing_docs)]

use proc_macro::TokenStream;

mod config;
mod parse_raw_string;
mod tr;
mod tr_load;
mod translation_file_cache;

/// Returns a translated string for the given key, and marks it for translation.
///
/// Examples:
/// ```rs
/// tr!("BUTTON_SIGN_IN", "Sign In");
/// tr!("BUTTON_LOG_OUT", "Log Out {{user}}", user=user.name);
/// ```
#[proc_macro]
pub fn tr(body: TokenStream) -> TokenStream {
    tr::tr(body)
}

/// Returns a translated, plural-matched string for the given key.
///
/// If default strings are provided, the amount of provided strings much match the amount of
/// strings required for the default language.
///
/// Examples:
/// ```rs
/// trn!(
///     "UNREAD_EMAILS",
///     "You have {{count}} unread email.",
///     "You have {{count}} unread emails.",
///     count=emails.len
/// );
///
/// trn!(
///     "USERS_ONLINE",
///     "{{count}} user online: {{users}}",
///     "{{count}} users online: {{users}}",
///     users=users.join(", "),
///     count=users.len
/// );
/// ```
#[proc_macro]
pub fn trn(body: TokenStream) -> TokenStream {
    tr::trn(body)
}

/// Generates an `I18nSource` containing the strings from the translation files located in the
/// configured translation directory.
///
/// By default, when strings are loaded (at compile time), the line endings are automatically
/// changed to match the compiled platform's line endings. If you wish to disable this behaviour,
/// set `match_line_endings` to false in your i18n configuration.
#[proc_macro]
pub fn tr_noop(_: TokenStream) -> TokenStream {
    TokenStream::default()
}

#[proc_macro]
pub fn trn_noop(_: TokenStream) -> TokenStream {
    TokenStream::default()
}

#[proc_macro]
pub fn tr_load(body: TokenStream) -> TokenStream {
    tr_load::tr_load(body)
}
