use crate::TrString;
use crate::tests::helpers::visit_string;

#[test]
fn simple_gen() {
    let visitor = visit_string(include_str!("simple_gen.rs.in"), "en");

    let hello_world = visitor
        .strings
        .get("HELLO_WORLD")
        .expect("HELLO_WORLD not found in parsed strings");
    assert_eq!(hello_world.line_no, 2);
    match &hello_world.string {
        TrString::Single(source_string) => {
            assert_eq!(source_string, "Hello World!")
        }
        TrString::Plural(_) => panic!("HELLO_WORLD is plural"),
    }
}
