mod date;
mod quote;
pub use date::Date;
pub use quote::Quote;

use crate::Locale;

pub type ModifierVariable<'a> = &'a (Option<&'a str>, &'a str);

pub trait StringModifier<T> {
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: T,
        variables: &'a [ModifierVariable<'a>],
    ) -> String;
}
