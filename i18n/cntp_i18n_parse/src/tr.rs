use std::hash::Hash;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

use crate::{MaybeFormattedNamedArg, NamedArg};

pub struct TrMacroInput {
    pub translation_id: LitStr,
    pub default_string: Option<LitStr>,
    pub variables: Punctuated<MaybeFormattedNamedArg, Token![,]>,
    pub context: Punctuated<NamedArg, Token![,]>,
}

impl Parse for TrMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let translation_id: LitStr = input.parse()?;

        let default_string = if input.peek2(syn::LitStr) {
            input.parse::<Token![,]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        let mut variables = Punctuated::new();
        let mut context = Punctuated::new();
        while !input.is_empty() {
            input.parse::<Token![,]>()?;

            if !input.is_empty() {
                let is_context = input.peek(Token![#]);

                if is_context {
                    input.parse::<Token![#]>()?;
                }

                if is_context {
                    let parse_result: NamedArg = input.parse()?;
                    context.push(parse_result);
                } else {
                    let parse_result: MaybeFormattedNamedArg = input.parse()?;
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

impl Hash for TrMacroInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.translation_id.hash(state);
        self.variables.hash(state);
        self.context.hash(state);
    }
}
