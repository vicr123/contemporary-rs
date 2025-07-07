use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

use crate::{MaybeFormattedNamedArg, NamedArg};

pub struct TrnMacroInput {
    pub translation_id: LitStr,
    pub default_strings: Vec<LitStr>,
    pub variables: Punctuated<MaybeFormattedNamedArg, Token![,]>,
    pub context: Punctuated<NamedArg, Token![,]>,
}

impl Parse for TrnMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let translation_id: LitStr = input.parse()?;
        // let default_string = if input.peek(syn::LitStr) {
        //     Some(input.parse()?)
        // } else {
        //     None
        // };

        let mut variables = Punctuated::new();
        let mut context = Punctuated::new();
        let mut default_strings: Vec<LitStr> = Vec::new();
        while !input.is_empty() {
            input.parse::<Token![,]>()?;

            if !input.is_empty() {
                let is_context = input.peek(Token![#]);

                if is_context {
                    input.parse::<Token![#]>()?;
                }

                if !is_context && !input.peek2(Token![=]) {
                    default_strings.push(input.parse()?)
                } else if is_context {
                    let parse_result: NamedArg = input.parse()?;
                    context.push(parse_result);
                } else {
                    let parse_result: MaybeFormattedNamedArg = input.parse()?;
                    variables.push(parse_result);
                }
            }
        }

        Ok(TrnMacroInput {
            translation_id,
            default_strings,
            variables,
            context,
        })
    }
}
