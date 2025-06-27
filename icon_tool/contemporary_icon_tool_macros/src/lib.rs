use contemporary_config::ContemporaryConfig;
use contemporary_icon_tool_core::contemporary_icon::ContemporaryIcon;
use proc_macro::TokenStream;
use quote::quote;
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

#[proc_macro]
pub fn application_icon(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as ApplicationIconMacroInput);

    let Some(config) = ContemporaryConfig::new_from_build_env() else {
        return Error::new(
            proc_macro::Span::call_site().into(),
            "Unable to read Contemporary.toml from build environment.",
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

    let Some(current_file) = proc_macro::Span::call_site().local_file() else {
        return Error::new(
            proc_macro::Span::call_site().into(),
            "Macro used in non-file context",
        )
        .to_compile_error()
        .into();
    };

    let icon_path = current_file.parent().unwrap().join(input.icon_file.value());
    if !icon_path.exists() {
        return Error::new(
            input.icon_file.span(),
            format!("Unable to find icon file: {}", icon_path.to_str().unwrap()),
        )
        .to_compile_error()
        .into();
    }

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
