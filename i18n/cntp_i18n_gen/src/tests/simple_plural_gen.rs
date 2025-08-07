use crate::TrString;
use crate::tests::helpers::visit_string;
use icu::plurals::PluralCategory;

#[test]
fn simple_plural_gen() {
    let visitor = visit_string(include_str!("simple_plural_gen.rs.in"), "en");

    let hello_world = visitor
        .strings
        .get("HELLO_WORLD")
        .expect("HELLO_WORLD not found in parsed strings");
    match &hello_world.string {
        TrString::Single(_) => {
            panic!("HELLO_WORLD is singular")
        }
        TrString::Plural(plural_strings) => {
            assert_eq!(plural_strings.len(), 2);
            let singular = plural_strings.first().unwrap();
            assert_eq!(singular.0, PluralCategory::One);
            assert_eq!(singular.1, "There are {{count}} world");
            let other = plural_strings.get(1).unwrap();
            assert_eq!(other.0, PluralCategory::Other);
            assert_eq!(other.1, "There are {{count}} worlds");
        }
    }
}
