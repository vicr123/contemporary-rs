use contemporary_i18n_parse::{tr::TrMacroInput, trn::TrnMacroInput};
use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;

use crate::config::I18N_CONFIG;

/// Returns a translated string for the given key.
/// Must be called in a context where a variable, `i18n` is available, which represents the
/// `I18nManager` to lookup the string from.
///
/// Examples:
/// ```rs
/// tr!("BUTTON_SIGN_IN", "Sign In");
/// tr!("BUTTON_LOG_OUT", "Log Out {{user}}", user=user.name);
/// ```
pub fn tr(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as TrMacroInput);

    let key = input.translation_id.value();
    quote! {
        i18n.lookup(#key, None)
    }
    .into()

    // if let Some(default_string) = input.default_string.map(|token| token.value()) {
    //     quote! { #default_string }.into()
    // } else {
    //     let key = input.translation_id.value();
    //     quote! { #key }.into()
    // }
}

/// Returns a translated, plural-matched string for the given key.
/// Must be called in a context where a variable, `i18n` is available, which represents the
/// `I18nManager` to lookup the string from.
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
pub fn trn(body: TokenStream) -> TokenStream {
    // let invocation_line = proc_macro::Span::call_site().start().line;

    let config = &*I18N_CONFIG;
    let input = parse_macro_input!(body as TrnMacroInput);

    quote! { "Pluralized String" }.into()
}

// tr!("KEY", "String", thing="a", other=thingy.into(), thing="a", #description="asdasd")
// i18n.lookup("KEY", )
// trn!("KEY", "String Zero", "String Singular", "String Plural", count=9, )
// trn!("KEY", "String Singular", "String Plural", "String Plural", count=9)
// tr_define!("KEY", "String", #description="asdasd")
