use super::ModifierVariable;
use crate::{Locale, modifiers::StringModifier};
use std::fmt::Display;

#[derive(Default)]
pub struct Quote;

impl<T> StringModifier<T> for Quote
where
    T: Display,
{
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: T,
        variables: &'a [ModifierVariable<'a>],
    ) -> String {
        if let Some((None, "alt")) = variables.first() {
            locale.quote_string_alternate(input)
        } else {
            locale.quote_string(input)
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use crate::{
        Locale,
        modifiers::{Quote, StringModifier},
    };

    #[test]
    fn string_modifier() {
        let locale = Locale::new_from_locale_identifier("en_US");
        let modifier = Quote;
        let result = modifier.transform(&locale, "Hello", &[]);
        assert_eq!(result, "\"Hello\"");
    }

    #[test]
    fn alternate_string_modifier() {
        let locale = Locale::new_from_locale_identifier("en_US");
        let modifier = Quote;
        let result = modifier.transform(&locale, "Hello", &[&(None, "alt")]);
        assert_eq!(result, "'Hello'");
    }
}
