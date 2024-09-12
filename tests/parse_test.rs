#[test]
fn test() {
    let try_test = trybuild::TestCases::new();
    try_test.pass("tests/as_ref.rs");
    try_test.pass("tests/from.rs");
    try_test.pass("tests/name.rs");
    try_test.pass("tests/new.rs");
    try_test.pass("tests/references.rs");
    try_test.pass("tests/references_generics.rs");
}
