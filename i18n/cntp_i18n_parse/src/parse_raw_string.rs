use crate::tr::TrMacroInput;
use crate::{MaybeFormattedNamedArg, NamedArg};
use std::hash::Hash;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

pub struct ParseRawStringMacroInput {
    pub raw_string: LitStr,
}

impl Parse for ParseRawStringMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let raw_string: LitStr = input.parse()?;

        Ok(ParseRawStringMacroInput { raw_string })
    }
}

impl Hash for ParseRawStringMacroInput {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.raw_string.hash(state);
    }
}
