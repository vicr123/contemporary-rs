pub mod tr;
pub mod trn;

use syn::{
    Expr, Ident, Token,
    parse::{Parse, ParseStream},
};

pub struct NamedArg {
    pub name: Ident,
    pub value: Expr,
}

impl Parse for NamedArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![=]>()?;
        let value: Expr = input.parse()?;
        Ok(NamedArg { name, value })
    }
}
