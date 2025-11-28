use crate::tests::helpers::visit_string;

#[test]
fn descriptions() {
    let visitor = visit_string(include_str!("descriptions.rs.in"), "en");

    let hello_world = visitor
        .strings
        .get("HELLO_WORLD")
        .expect("HELLO_WORLD not found in parsed strings");
    assert_eq!(
        hello_world
            .description
            .as_ref()
            .expect("HELLO_WORLD has no description"),
        "\"Hello World\" is a simple greeting phrase commonly used as the first output in programming tutorials and examples."
    )
}
