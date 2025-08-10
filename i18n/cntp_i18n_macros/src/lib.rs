use proc_macro::TokenStream;

mod config;
mod preprocessor;
mod tr;
mod tr_load;

#[proc_macro]
pub fn tr(body: TokenStream) -> TokenStream {
    tr::tr(body)
}

#[proc_macro]
pub fn trn(body: TokenStream) -> TokenStream {
    tr::trn(body)
}

#[proc_macro]
pub fn tr_load(body: TokenStream) -> TokenStream {
    tr_load::tr_load(body)
}
