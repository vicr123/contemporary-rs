use crate::VisitorErrorType;
use crate::tests::helpers::visit_string;

#[test]
fn duplicates() {
    let visitor = visit_string(include_str!("duplicates.rs.in"), "en");

    assert_eq!(visitor.errors.len(), 2);
    let error = &visitor.errors[0];
    match &error.error_type {
        VisitorErrorType::DuplicateDefinition {
            id,
            last_seen_file: _,
            last_seen_line,
        } => {
            assert_eq!(id, "HELLO_WORLD");
            assert_eq!(last_seen_line, &2);
        }
        _ => panic!("Visitor returned incorrect error type"),
    }
    let error = &visitor.errors[1];
    match &error.error_type {
        VisitorErrorType::DuplicateDefinition {
            id,
            last_seen_file: _,
            last_seen_line,
        } => {
            assert_eq!(id, "HELLO_WORLD_PLURAL");
            assert_eq!(last_seen_line, &4);
        }
        _ => panic!("Visitor returned incorrect error type"),
    }
}
