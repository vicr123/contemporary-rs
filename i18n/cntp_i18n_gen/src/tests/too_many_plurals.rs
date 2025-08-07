use crate::VisitorErrorType;
use crate::tests::helpers::visit_string;

#[test]
fn too_many_plurals() {
    let visitor = visit_string(include_str!("too_many_plurals.rs.in"), "en");

    assert_eq!(visitor.errors.len(), 1);
    let error = &visitor.errors[0];
    match &error.error_type {
        VisitorErrorType::BadPluralArgumentCount {
            id,
            expected_count,
            actual_count,
        } => {
            assert_eq!(id, "HELLO_WORLD");
            assert_eq!(expected_count, &2);
            assert_eq!(actual_count, &3);
        }
        _ => panic!("Visitor returned incorrect error type"),
    }
}
