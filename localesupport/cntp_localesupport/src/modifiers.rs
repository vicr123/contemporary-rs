mod quote;
pub use quote::Quote;

use crate::Locale;

type ModifierVariable<'a> = &'a (Option<&'a str>, &'a str);

pub trait StringModifier<T>: Default {
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: T,
        variables: &'a [ModifierVariable<'a>],
    ) -> String;
}
