use crate::FormatterInvocation;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, Token};

pub struct TrfMacroInput {
    pub formatters: Punctuated<FormatterInvocation, Token![,]>,
    pub variable: Expr,
}

impl Parse for TrfMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut formatters: Punctuated<FormatterInvocation, Token![,]> = Punctuated::new();
        while !input.is_empty() {
            let fork = input.fork();
            if fork.parse::<FormatterInvocation>().is_err() {
                break;
            }

            let Ok(comma) = fork.parse::<Token![,]>() else {
                break;
            };

            formatters.push_value(input.parse()?);
            input.parse::<Token![,]>()?;

            if fork.parse::<FormatterInvocation>().is_ok() {
                formatters.push_punct(comma);
            } else {
                break;
            }
        }

        let variable = input.parse()?;

        Ok(TrfMacroInput {
            formatters,
            variable,
        })
    }
}
