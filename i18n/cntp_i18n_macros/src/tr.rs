use cntp_i18n_parse::{
    MaybeFormattedNamedArg, MaybeNamedFormatterArg, tr::TrMacroInput, trn::TrnMacroInput,
};
use proc_macro::TokenStream;

use quote::quote;
use syn::{Error, Ident, Path, parse_macro_input, punctuated::Punctuated, token::Comma};

use crate::config::CURRENT_CRATE;

pub fn resolve_modifier(path: Path) -> proc_macro2::TokenStream {
    if let Some(ident) = path.get_ident() {
        let name = ident.to_string();

        match name.as_str() {
            "quote" => quote! { cntp_i18n::Quote },
            _ => quote! { #path },
        }
    } else {
        quote! { #path }
    }
}

pub fn non_count_variable(
    variable: MaybeFormattedNamedArg,
    bsmi_decls: &mut Vec<proc_macro2::TokenStream>,
    ssmi_decls: &mut Vec<proc_macro2::TokenStream>,
    z: &mut Vec<proc_macro2::TokenStream>,
) {
    let bsmi_ref = if let Some(mod_invoke) = variable.formatters.first() {
        let bsmi_name: Ident = Ident::new(
            &format!("bsmi_decl_{}", bsmi_decls.len()),
            proc_macro2::Span::call_site(),
        );
        let bsmi_path = resolve_modifier(mod_invoke.name.clone());
        let bsmi_expr = variable.value.clone();

        let bsmi_vars = create_mi_vars(&mod_invoke.args);

        let bsmi_decl = quote! {
            let #bsmi_name = BaseStringModifierInvocation::new(
                &#bsmi_path,
                #bsmi_vars,
                &#bsmi_expr
            );
        };

        bsmi_decls.push(bsmi_decl);

        Some(quote! {
            (&#bsmi_name as &dyn ErasedStringModifierTransform)
        })
    } else {
        None
    };

    let ssmis = variable
        .formatters
        .iter()
        .skip(1)
        .map(|mod_invoke| {
            let path = resolve_modifier(mod_invoke.name.clone());
            let args = create_mi_vars(&mod_invoke.args);

            quote! {
                SubsequentStringModifierInvocation::new(
                    &#path,
                    #args,
                ),
            }
        })
        .collect::<Vec<_>>();

    let var_name = variable.name.to_string();

    if let Some(bsmi_ref) = bsmi_ref {
        let ssmi_decl_name: Ident = Ident::new(
            &format!("ssmi_decl_{}", ssmi_decls.len()),
            proc_macro2::Span::call_site(),
        );

        let ssmi_decl = quote! {
            let #ssmi_decl_name = [#( #ssmis )*];
        };

        ssmi_decls.push(ssmi_decl);

        z.push(quote! {
            (
                #var_name,
                {
                    use cntp_i18n::Variable;
                    Variable::Modified(#bsmi_ref, &#ssmi_decl_name)
                }
            ),
        });
    } else {
        let expr = variable.value;
        z.push(quote! {
            (
                #var_name,
                {
                    use cntp_i18n::Variable;
                    Variable::String(#expr.to_string())
                }
            ),
        });
    }
}

pub fn create_mi_vars(
    args: &Punctuated<MaybeNamedFormatterArg, Comma>,
) -> proc_macro2::TokenStream {
    let bsmi_vars: Vec<_> = args
        .iter()
        .map(|arg| {
            let name = if let Some(name) = arg.name.clone() {
                quote! { Some(#name) }
            } else {
                quote! { None }
            };
            let value = arg.value.value();

            quote! { &(#name, #value), }
        })
        .collect();

    quote! {
        &[ #( #bsmi_vars )* ]
    }
}

/// Returns a translated string for the given key.
///
/// Examples:
/// ```rs
/// tr!("BUTTON_SIGN_IN", "Sign In");
/// tr!("BUTTON_LOG_OUT", "Log Out {{user}}", user=user.name);
/// ```
pub fn tr(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as TrMacroInput);

    let mut bsmi_decls = Vec::new();
    let mut ssmi_decls = Vec::new();

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
                        "Unused translation variable {} specified when rendering key {}",
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
                        use cntp_i18n::Variable;
                        Variable::Count(#expr)
                    }
                ),
            });
        } else {
            non_count_variable(variable, &mut bsmi_decls, &mut ssmi_decls, &mut z);
        }
    }

    let key = input.translation_id.value();
    let current_crate = &*CURRENT_CRATE;
    let token_length = z.len();

    quote! {
        {
            use cntp_i18n::I18N_MANAGER as i18n;
            use cntp_i18n::{Variable, BaseStringModifierInvocation, ErasedStringModifierTransform,
                SubsequentStringModifierInvocation, StringModifier};

            #( #bsmi_decls )*
            #( #ssmi_decls )*

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

    let mut bsmi_decls = Vec::new();
    let mut ssmi_decls = Vec::new();

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
                    "Unused translation variable {} specified when rendering key {}",
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
                        use cntp_i18n::Variable;
                        Variable::Count(#expr)
                    }
                ),
            });
        } else {
            non_count_variable(variable, &mut bsmi_decls, &mut ssmi_decls, &mut z);
        }
    }

    let key = input.translation_id.value();
    let current_crate = &*CURRENT_CRATE;
    let token_length = z.len();

    quote! {
        {
            use cntp_i18n::I18N_MANAGER as i18n;
            use cntp_i18n::{Variable, BaseStringModifierInvocation, ErasedStringModifierTransform,
                SubsequentStringModifierInvocation, StringModifier};

            #( #bsmi_decls )*
            #( #ssmi_decls )*

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
