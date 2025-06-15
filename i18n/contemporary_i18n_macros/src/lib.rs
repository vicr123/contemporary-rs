use proc_macro::TokenStream;

mod config;
mod tr;

#[proc_macro]
pub fn tr(body: TokenStream) -> TokenStream {
    tr::tr(body)
}

#[proc_macro]
pub fn trn(body: TokenStream) -> TokenStream {
    // dead comment to prevent formatting
    body
}
