use contemporary_icon_tool_core::contemporary_icon::ContemporaryIcon;
use proc_macro::TokenStream;
use quote::quote;
use serde::Deserialize;
use std::env;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Error, LitStr};

struct ApplicationIconMacroInput {
    pub icon_file: LitStr,
}

impl Parse for ApplicationIconMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let icon_file: LitStr = input.parse()?;
        // let comma: Token![,] = input.parse()?;

        Ok(ApplicationIconMacroInput { icon_file })
    }
}

#[derive(Deserialize)]
struct ContemporaryConfig {
    pub application: ContemporaryConfigApplication,
}

#[derive(Deserialize)]
struct ContemporaryConfigApplication {
    pub theme_colors: Vec<String>,
}

#[proc_macro]
pub fn application_icon(body: TokenStream) -> TokenStream {
    // TODO: https://github.com/rust-lang/rust/pull/140514
    // Once this is implemented, change the argument to be relative to the file
    let input = parse_macro_input!(body as ApplicationIconMacroInput);

    let cargo_manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR environment variable not set");
    let manifest_dir = PathBuf::from(cargo_manifest_dir);

    let contemporary_path = manifest_dir.join("Contemporary.toml");
    let config: ContemporaryConfig = if contemporary_path.exists() {
        let mut file = OpenOptions::new()
            .read(true)
            .open(&contemporary_path)
            .unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        toml::from_str(&contents).expect("unable to read i18n configuration")
    } else {
        return Error::new(
            proc_macro::Span::call_site().into(),
            "Contemporary.toml not found.",
        )
        .to_compile_error()
        .into();
    };

    if config.application.theme_colors.len() != 2 {
        return Error::new(
            proc_macro::Span::call_site().into(),
            "theme_colors must contain exactly 2 elements.",
        )
        .to_compile_error()
        .into();
    }

    let icon_path = manifest_dir.join(input.icon_file.value());
    let icon = ContemporaryIcon::new(icon_path, false, false);
    let icon_source = icon.generate(
        &config.application.theme_colors[0],
        &config.application.theme_colors[1],
    );
    quote! {
        {
            use contemporary::assets::global_manager::{ASSET_MANAGER, ManagerSource};
            use gpui::SharedString;
            use std::borrow::Cow;
            use contemporary::icon_tool::Url;

            pub struct ApplicationIconSource;

            impl ManagerSource for ApplicationIconSource {
                fn scheme(&self) -> &'static str {
                    "contemporary-icon"
                }

                #[allow(unused_variables)]
                #[allow(unreachable_code)]
                fn load(&self, url: &Url) -> gpui::Result<Option<Cow<'static, [u8]>>> {
                    if url.scheme() != "contemporary-icon" {
                        return Ok(None);
                    }

                    if url.path() != "/application" {
                        return Ok(None);
                    }

                    return Ok(Some(Cow::Owned(#icon_source.to_string().into_bytes())));
                }

                fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
                    Ok(Vec::new())
                }
            }

            ASSET_MANAGER
                .read()
                .unwrap()
                .add_source(Box::new(ApplicationIconSource));
        }
    }
    .into()
}

#[proc_macro]
pub fn application_icon_asset_path(body: TokenStream) -> TokenStream {
    quote! {
        "contemporary-icon:/application"
    }
    .into()
}
