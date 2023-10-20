#![recursion_limit = "256"]

#[test]
fn test_success() {
    let t = trybuild::TestCases::new();
    t.pass("tests/cases/*_success.rs");
}
