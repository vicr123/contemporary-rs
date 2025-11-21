use quote::quote;

fn match_line_endings(string: &str) -> proc_macro2::TokenStream {
    if true {
        let windows_string = string.replace("\n", "\r\n");
        let unix_string = string.replace("\r\n", "\n");

        return quote! {
            {
                #[cfg(target_os = "windows")]
                {#windows_string}

                #[cfg(not(target_os = "windows"))]
                {#unix_string}
            }
        };
    }

    quote! { #string }
}

pub fn parse_raw_string(string: &str) -> proc_macro2::TokenStream {
    let normalised = match_line_endings(string);

    quote! {
        {
            &[cntp_i18n::I18nStringPart::BorrowedStatic(#normalised)]
        }
    }
}
