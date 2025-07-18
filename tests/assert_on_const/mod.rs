use crate::test_lint_diagnostics;

#[test]
fn assert_on_bool_literal() {
    test_lint_diagnostics!(r#"
    fn foo() {
        assert!(true, "message");
    }
    "#, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:17
            assert!(true, "message");
                    ^^^^
    "#)
}

#[test]
fn assert_on_bool_const() {
    test_lint_diagnostics!(r"
    const C: bool = false;
    fn foo() {
        assert!(C);
    }
    ", @r"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:4:17
    assert!(C);
            ^
    ")
}
