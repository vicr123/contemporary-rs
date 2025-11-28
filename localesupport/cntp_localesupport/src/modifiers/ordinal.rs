use crate::Locale;
use crate::modifiers::{ModifierVariable, StringModifier};
use icu::plurals::PluralRules;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Default)]
pub struct Ordinal;

impl<T> StringModifier<T> for Ordinal
where
    T: Display,
{
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: T,
        variables: &'a [ModifierVariable<'a>],
    ) -> String {
        // Panic if input is not a number
        let input = i32::from_str(input.to_string().as_str()).unwrap();
        let plural_rules = PluralRules::try_new_ordinal(locale.numeric_icu.clone().into()).unwrap();

        format!("{input}th")
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use crate::{
        Locale,
        modifiers::{Ordinal, StringModifier},
    };

    #[test]
    fn ordinals_en() {
        let locale = Locale::new_from_locale_identifier("en_US");
        let modifier = Ordinal;
        let result = modifier.transform(&locale, "0", &[]);
        assert_eq!(result, "0th");
        let result = modifier.transform(&locale, "1", &[]);
        assert_eq!(result, "1st");
        let result = modifier.transform(&locale, "2", &[]);
        assert_eq!(result, "2nd");
        let result = modifier.transform(&locale, "3", &[]);
        assert_eq!(result, "3rd");
        let result = modifier.transform(&locale, "4", &[]);
        assert_eq!(result, "4th");
        let result = modifier.transform(&locale, "5", &[]);
        assert_eq!(result, "5th");
        let result = modifier.transform(&locale, "6", &[]);
        assert_eq!(result, "6th");
        let result = modifier.transform(&locale, "7", &[]);
        assert_eq!(result, "7th");
        let result = modifier.transform(&locale, "8", &[]);
        assert_eq!(result, "8th");
        let result = modifier.transform(&locale, "9", &[]);
        assert_eq!(result, "9th");
        let result = modifier.transform(&locale, "10", &[]);
        assert_eq!(result, "10th");
        let result = modifier.transform(&locale, "11", &[]);
        assert_eq!(result, "11th");
        let result = modifier.transform(&locale, "12", &[]);
        assert_eq!(result, "12th");
        let result = modifier.transform(&locale, "13", &[]);
        assert_eq!(result, "13th");
        let result = modifier.transform(&locale, "20", &[]);
        assert_eq!(result, "20th");
        let result = modifier.transform(&locale, "21", &[]);
        assert_eq!(result, "21st");
        let result = modifier.transform(&locale, "22", &[]);
        assert_eq!(result, "22nd");
        let result = modifier.transform(&locale, "23", &[]);
        assert_eq!(result, "23rd");
    }
}
