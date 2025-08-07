use crate::VisitorErrorType;
use crate::tests::helpers::visit_string;

#[test]
fn predefine() {
    let visitor = visit_string(include_str!("predefine.rs.in"), "en");

    assert!(visitor.strings.contains_key("HELLO_WORLD"));
    assert!(visitor.strings.contains_key("HELLO_WORLD_PLURAL"));

    assert_eq!(visitor.errors.len(), 2);
    let first_error = &visitor.errors[0];
    match &first_error.error_type {
        VisitorErrorType::MissingDefinition { id } => {
            assert_eq!(id, "THIS_STRING_IS_NOT_DEFINED");
        }
        _ => panic!("Visitor returned incorrect error type"),
    }
    let second_error = &visitor.errors[1];
    match &second_error.error_type {
        VisitorErrorType::MissingDefinition { id } => {
            assert_eq!(id, "THIS_PLURAL_STRING_IS_NOT_DEFINED");
        }
        _ => panic!("Visitor returned incorrect error type"),
    }
}
