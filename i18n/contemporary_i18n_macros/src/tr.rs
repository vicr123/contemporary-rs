use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use proc_macro::TokenStream;

use quote::quote;
use serde_json::{Value, json};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token, parse_macro_input};

use crate::config::I18N_CONFIG;

static IS_TRANSLATION_CATALOG_ERASED: Lazy<Arc<Mutex<bool>>> =
    Lazy::new(|| Arc::new(Mutex::new(false)));

struct TrMacroInput {
    translation_id: LitStr,
    default_string: Option<LitStr>,
    variables: Punctuated<NamedArg, Token![,]>,
    context: Punctuated<NamedArg, Token![,]>,
}

struct NamedArg {
    name: Ident,
    value: Expr,
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;
        Ok(NamedArg { name, value })
    }
}

impl Parse for TrMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let translation_id: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let default_string = if input.peek(syn::LitStr) {
            Some(input.parse()?)
        } else {
            None
        };

        let mut variables = Punctuated::new();
        let mut context = Punctuated::new();
        while !input.is_empty() {
            input.parse::<Token![,]>()?;

            let is_context = input.peek(Token![#]);

            if is_context {
                input.parse::<Token![#]>()?;
            }

            if !input.is_empty() {
                let parse_result: NamedArg = input.parse()?;
                if is_context {
                    context.push(parse_result);
                } else {
                    variables.push(parse_result);
                }
            }
        }

        Ok(TrMacroInput {
            translation_id,
            default_string,
            variables,
            context,
        })
    }
}

struct Meta {
    description: Option<String>,
}

fn translation_directory() -> PathBuf {
    // Get the project root from the CARGO_MANIFEST_DIR environment variable
    let project_root = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let mut file_path = PathBuf::from(&project_root);
    file_path.push("translations");

    fs::create_dir_all(&file_path).expect("Unable to create translations directory");

    file_path
}

fn translation_meta_file() -> PathBuf {
    let mut file_path = translation_directory();
    file_path.push("meta.json");

    file_path
}

fn translation_catalog_file() -> PathBuf {
    let config = &*I18N_CONFIG;

    let mut file_path = translation_directory();
    file_path.push(format!("{}.json", config.i18n.default_language));

    file_path
}

fn insert_into_translation_catalog(key: &str, value: Value) {
    let mut catalog_erased = IS_TRANSLATION_CATALOG_ERASED
        .lock()
        .expect("could not lock catalog erased state");

    let file_path = translation_catalog_file();

    // Read the existing JSON file or create a new one
    let mut translations: Value = if file_path.exists() && *catalog_erased {
        let mut file = OpenOptions::new().read(true).open(&file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or_else(|_| json!({}))
    } else {
        *catalog_erased = true;
        json!({})
    };

    // Insert or update the translation key with the default string
    translations[key] = value;

    // Write back to the file
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();
    file.write_all(
        serde_json::to_string_pretty(&translations)
            .unwrap()
            .as_bytes(),
    )
    .unwrap();
    
    // this needs to live for the whole duration of the function
    drop(catalog_erased)
}

/// Returns a translated string for the given key.
/// Must be called in a context where a variable, `i18n` is available, which represents the `I18nManager` to lookup the string from.
///
/// Examples:
/// ```rs
/// tr!("BUTTON_SIGN_IN", "Sign In")
/// tr!("BUTTON_LOG_OUT", "Log Out {{user}}", user=user.name)
/// ```
pub fn tr(body: TokenStream) -> TokenStream {
    // let invocation_line = proc_macro::Span::call_site().start().line;

    let config = &*I18N_CONFIG;
    let input = parse_macro_input!(body as TrMacroInput);

    if let Some(default_string) = input.default_string.map(|token| token.value()) {
        // TODO: Ensure this form of the macro is only used once and throw an error otherwise
        if config.i18n.generate_translation_files {
            insert_into_translation_catalog(
                input.translation_id.value().as_str(),
                json!(default_string),
            );
        };

        quote! { #default_string }.into()
    } else {
        let key = input.translation_id.value();
        quote! { #key }.into()
    }
}

// tr!("KEY", "String", thing="a", other=thingy.into(), thing="a", #description="asdasd")
// i18n.lookup("KEY", )
// trn!("KEY", "String Singular", "String Plural", count=9, )
// trn!("KEY", "String Singular", "String Plural", "String Plural", count=9)
// tr_define!("KEY", "String", #description="asdasd")
