use cntp_config::ContemporaryConfig;
use proc_macro::TokenStream;
use quote::quote;
use std::env;
use syn::Error;

#[proc_macro]
pub fn application_details(_: TokenStream) -> TokenStream {
    let Some(config) = ContemporaryConfig::new_from_build_env() else {
        return Error::new(
            proc_macro::Span::call_site().into(),
            "Unable to read Contemporary.toml from build environment.",
        )
        .to_compile_error()
        .into();
    };

    let target_triple = env::var("CARGO_BUILD_TARGET").unwrap_or_else(|_| {
        env::var("TARGET").unwrap_or_else(|_| {
            // Guess the build target - getting the absolutely correct one isn't too important unless some items are set differently
            if cfg!(target_os = "macos") {
                if cfg!(target_arch = "aarch64") {
                    "aarch64-apple-darwin".into()
                } else {
                    "x86_64-apple-darwin".into()
                }
            } else if cfg!(target_os = "windows") {
                "x86_64-pc-windows-msvc".into()
            } else {
                "x86_64-unknown-linux-gnu".into()
            }
        })
    });

    let deployment = config.deployment(target_triple.as_str());

    let application_name = serde_json::to_string(&deployment.application_name()).unwrap();
    let generic_name = serde_json::to_string(&deployment.application_generic_name).unwrap();
    let desktop_entry = deployment.desktop_entry.unwrap_or("".into());
    let application_machine_name = match deployment.application_machine_name {
        None => quote! { None },
        Some(application_machine_name) => quote! { Some(#application_machine_name) },
    };
    let organization_name = match deployment.organization_name {
        None => quote! { None },
        Some(org_name) => quote! { Some(#org_name) },
    };

    quote! {
        {
            contemporary::application::GeneratableDetails {
                application_name: contemporary::macros::from_str(#application_name).unwrap(),
                application_generic_name: contemporary::macros::from_str(#generic_name).unwrap(),
                application_machine_name: #application_machine_name,
                organization_name: #organization_name,
                desktop_entry: #desktop_entry,
            }
        }
    }
    .into()
}
