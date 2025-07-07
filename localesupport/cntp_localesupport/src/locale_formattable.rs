use crate::Locale;
use icu::decimal::input::Decimal;
use std::str::FromStr;

pub trait LocaleFormattable {
    fn to_locale_string(self, locale: &Locale) -> String;
}

macro_rules! locale_formattable_integer_impl {
    ($typ:ty) => {
        impl LocaleFormattable for $typ {
            fn to_locale_string(self, locale: &Locale) -> String {
                locale.format_decimal(Decimal::from(self))
            }
        }
    };
}

locale_formattable_integer_impl!(i8);
locale_formattable_integer_impl!(i16);
locale_formattable_integer_impl!(i32);
locale_formattable_integer_impl!(i64);
locale_formattable_integer_impl!(i128);
locale_formattable_integer_impl!(u8);
locale_formattable_integer_impl!(u16);
locale_formattable_integer_impl!(u32);
locale_formattable_integer_impl!(u64);
locale_formattable_integer_impl!(u128);

macro_rules! locale_formattable_stringable_impl {
    ($typ:ty) => {
        impl LocaleFormattable for $typ {
            fn to_locale_string(self, locale: &Locale) -> String {
                locale.format_decimal(Decimal::from_str(self.to_string().as_str()).unwrap())
            }
        }
    };
}

locale_formattable_stringable_impl!(f32);
locale_formattable_stringable_impl!(f64);
