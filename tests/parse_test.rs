#[test]
fn test() {
    let try_test = trybuild::TestCases::new();
    try_test.pass("tests/from.rs");
    try_test.pass("tests/as_ref.rs");
    try_test.pass("tests/references.rs");
}
