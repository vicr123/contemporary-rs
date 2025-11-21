pub mod tr;
pub mod trn;

use syn::{
    Expr, Ident, LitStr, Path, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

#[derive(Hash)]
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

#[derive(Hash)]
pub struct MaybeFormattedNamedArg {
    pub name: Ident,
    pub value: Expr,
    pub formatters: Punctuated<FormatterInvocation, Token![:]>,
    pub use_locale_string: bool,
}

impl Parse for MaybeFormattedNamedArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let mut formatters = Punctuated::new();
        while input.peek(Token![:]) && !input.is_empty() {
            input.parse::<Token![:]>()?;
            formatters.push(input.parse()?);
        }

        input.parse::<Token![=]>()?;

        let use_locale_string = if input.peek(Token![!]) {
            input.parse::<Token![!]>()?;
            false
        } else {
            true
        };

        let value: Expr = input.parse()?;

        Ok(MaybeFormattedNamedArg {
            name,
            value,
            formatters,
            use_locale_string,
        })
    }
}

#[derive(Hash)]
pub struct MaybeNamedFormatterArg {
    pub name: Option<Ident>,
    pub value: LitStr,
}

impl Parse for MaybeNamedFormatterArg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = if input.peek(Ident) {
            let name = input.parse()?;
            input.parse::<Token![=]>()?;
            Some(name)
        } else {
            None
        };

        let value: LitStr = input.parse()?;
        Ok(MaybeNamedFormatterArg { name, value })
    }
}

#[derive(Hash)]
pub struct FormatterInvocation {
    pub name: Path,
    pub args: Punctuated<MaybeNamedFormatterArg, Token![,]>,
}

impl Parse for FormatterInvocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Path = input.parse()?;
        let args = if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            Some(content.parse_terminated(MaybeNamedFormatterArg::parse, Token![,])?)
        } else {
            None
        };

        let args = args.unwrap_or_default();
        Ok(FormatterInvocation { name, args })
    }
}
