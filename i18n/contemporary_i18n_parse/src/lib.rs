use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Expr, Ident, LitStr, Token};

pub struct TrMacroInput {
    pub translation_id: LitStr,
    pub default_string: Option<LitStr>,
    pub variables: Punctuated<NamedArg, Token![,]>,
    pub context: Punctuated<NamedArg, Token![,]>,
}

pub struct NamedArg {
    pub name: Ident,
    pub value: Expr,
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
