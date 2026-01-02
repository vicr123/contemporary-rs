//! # Contemporary i18n Parser
//!
//! This crate provides parsing utilities for the Contemporary i18n macro system.
//! It defines the syntax and parsing logic for `tr!` and `trn!` macro invocations.
//!
//! ## Overview
//!
//! This is an internal crate used by `cntp_i18n_macros` and `cntp_i18n_gen` to parse
//! the arguments passed to translation macros. Most users will not need to use this
//! crate directly.
//!
//! ## Modules
//!
//! - [`tr`] - Parsing for the `tr!` macro (simple translations)
//! - [`trn`] - Parsing for the `trn!` macro (plural translations)
//!
//! ## Syntax Elements
//!
//! The parser handles several syntax elements:
//!
//! - **Named arguments**: `name = value`
//! - **Formatted arguments**: `name:Modifier = value` or `name:Modifier(arg) = value`
//! - **Modifier chains**: `name:Quote:Date("YMD") = value`
//! - **Literal suppression**: `name = !value` (disables locale formatting)
//!
//! ## Example Macro Syntax
//!
//! ```rust,ignore
//! // Simple translation
//! tr!("KEY", "Default text");
//!
//! // With variables
//! tr!("KEY", "Hello {{name}}", name = user_name);
//!
//! // With modifiers
//! tr!("KEY", "Date: {{date}}", date:Date("YMD") = timestamp);
//!
//! // Plural translation
//! trn!("KEY", "{{count}} item", "{{count}} items", count = item_count);
//! ```

pub mod tr;
pub mod trn;

use syn::{
    Expr, Ident, LitStr, Path, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

#[derive(Hash)]
/// A simple named argument in a translation macro.
///
/// Represents syntax like `name = value` in a `tr!` or `trn!` macro call.
pub struct NamedArg {
    /// The argument name (left side of `=`).
    pub name: Ident,
    /// The argument value expression (right side of `=`).
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

/// A named argument that may have formatting modifiers applied.
///
/// Represents syntax like `name:Modifier:Modifier2 = value` or `name = !value`.
///
/// # Examples
///
/// - `date:Date("YMD") = timestamp` - Apply Date modifier with "YMD" format
/// - `name:Quote = user_name` - Apply Quote modifier
/// - `raw = !some_value` - Disable automatic locale string conversion
#[derive(Hash)]
pub struct MaybeFormattedNamedArg {
    /// The argument name.
    pub name: Ident,
    /// The argument value expression.
    pub value: Expr,
    /// Chain of formatters/modifiers to apply (e.g., `:Quote:Date`).
    pub formatters: Punctuated<FormatterInvocation, Token![:]>,
    /// Whether to use locale string conversion (`true` by default, `false` if `!` prefix).
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

/// An argument to a formatter/modifier, which may be named or positional.
///
/// Represents arguments inside modifier parentheses like `Date(format = "YMD")` or `Date("YMD")`.
#[derive(Hash)]
pub struct MaybeNamedFormatterArg {
    /// The argument name, if provided (e.g., `format` in `format = "YMD"`).
    pub name: Option<Ident>,
    /// The string literal value.
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

/// A formatter/modifier invocation in a translation macro.
///
/// Represents syntax like `Date("YMD")` or `Quote` in a modifier chain.
///
/// # Examples
///
/// - `Quote` - Modifier with no arguments
/// - `Date("YMD")` - Modifier with positional argument
/// - `Date(format = "YMD", length = "short")` - Modifier with named arguments
#[derive(Hash)]
pub struct FormatterInvocation {
    /// The path to the modifier type (e.g., `Quote`, `Date`, `my_mod::Custom`).
    pub name: Path,
    /// Arguments passed to the modifier in parentheses.
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
