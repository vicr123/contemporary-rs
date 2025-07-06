use contemporary_i18n_parse::{tr::TrMacroInput, trn::TrnMacroInput};
use proc_macro::TokenStream;

use quote::quote;
use syn::{Error, parse_macro_input};

use crate::config::CURRENT_CRATE;

/// Returns a translated string for the given key.
///
/// Examples:
/// ```rs
/// tr!("BUTTON_SIGN_IN", "Sign In");
/// tr!("BUTTON_LOG_OUT", "Log Out {{user}}", user=user.name);
/// ```
pub fn tr(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as TrMacroInput);

    let mut z = Vec::new();
    for variable in input.variables {
        // Ensure the variable is used
        if let Some(default_string) = &input.default_string {
            if !default_string
                .value()
                .contains(format!("{{{{{}}}}}", variable.name).as_str())
            {
                return Error::new(
                    variable.name.span(),
                    format!(
                        "Translation variable {} specified when rendering key {} but not used",
                        variable.name,
                        input.translation_id.value()
                    ),
                )
                .to_compile_error()
                .into();
            }
        }

        if variable.name == "count" {
            // Special handling
            let var_name = variable.name.to_string();
            let expr = variable.value;
            z.push(quote! {
                (
                    #var_name,
                    {
                        use contemporary_i18n::Variable;
                        Variable::Count(#expr)
                    }
                ),
            });
        } else {
            let var_name = variable.name.to_string();
            let expr = variable.value;
            z.push(quote! {
                (
                    #var_name,
                    {
                        use contemporary_i18n::Variable;
                        Variable::String(#expr.to_string())
                    }
                ),
            });
        }
    }

    let key = input.translation_id.value();
    let current_crate = &*CURRENT_CRATE;
    let token_length = z.len();

    quote! {
        {
            use contemporary_i18n::I18N_MANAGER as i18n;
            use contemporary_i18n::Variable;
            i18n.read().unwrap().lookup::<[(&'_ str, Variable); #token_length]>(#key, &[
                #( #z )*
            ], #current_crate)
        }
    }
    .into()
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
pub fn trn(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as TrnMacroInput);

    let mut z = Vec::new();
    for variable in input.variables {
        let is_used = input.default_strings.iter().any(|default_string| {
            default_string
                .value()
                .contains(format!("{{{{{}}}}}", variable.name).as_str())
        });
        if !is_used {
            return Error::new(
                variable.name.span(),
                format!(
                    "Translation variable {} specified when rendering key {} but not used",
                    variable.name,
                    input.translation_id.value()
                ),
            )
            .to_compile_error()
            .into();
        }

        if variable.name == "count" {
            // Special handling
            let var_name = variable.name.to_string();
            let expr = variable.value;
            z.push(quote! {
                (
                    #var_name,
                    {
                        use contemporary_i18n::Variable;
                        Variable::Count(#expr)
                    }
                ),
            });
        } else {
            let var_name = variable.name.to_string();
            let expr = variable.value;
            z.push(quote! {
                (
                    #var_name,
                    {
                        use contemporary_i18n::Variable;
                        Variable::String(#expr.to_string())
                    }
                ),
            });
        }
    }

    let key = input.translation_id.value();
    let current_crate = &*CURRENT_CRATE;
    let token_length = z.len();

    quote! {
        {
            use contemporary_i18n::I18N_MANAGER as i18n;
            use contemporary_i18n::Variable;
            i18n.read().unwrap().lookup::<[(&'_ str, Variable); #token_length]>(#key, &[
                #( #z )*
            ], #current_crate)
        }
    }
    .into()
}

// tr!("KEY", "String", thing="a", other=thingy.into(), thing="a", #description="asdasd")
// i18n.lookup("KEY", )
// trn!("KEY", "String Zero", "String Singular", "String Plural", count=9, )
// trn!("KEY", "String Singular", "String Plural", "String Plural", count=9)
// tr_define!("KEY", "String", #description="asdasd")
