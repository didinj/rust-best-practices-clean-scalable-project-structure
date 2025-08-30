use rust_project_structure_demo::greet;

#[test]
fn test_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
