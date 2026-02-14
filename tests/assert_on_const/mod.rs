use crate::test_lint_diagnostics;

const BOOL_LITERAL: &str = r#"
fn foo() {
    assert!(true, "message");
}
"#;

const BOOL_CONST: &str = r#"
const C: bool = false;
fn foo() {
    assert!(C, "message");
}
"#;

const BOOL_CONST_NEGATED: &str = r#"
const C: bool = false;
fn foo() {
    assert!(!C, "message");
}
"#;

const BOOL_EXPR_SIMPLE: &str = r#"
fn foo() {
    assert!(1 == 1, "message");
}
"#;

const BOOL_EXPR_SIMPLE_NEGATED: &str = r#"
fn foo() {
    assert!(!((1 == 1) && (2 == 2)), "message");
}
"#;

const BOOL_EXPR_ARITHMETIC: &str = r#"
fn foo() {
    assert!(1 == (1 + 1), "message");
}
"#;

const BOOL_EXPR_ARITHMETIC_COMPLEX: &str = r#"
fn foo() {
assert!((1 == 1) && (5 - 5 == 3 - 3));
}
"#;

const BOOL_EXPR_WITH_CONST: &str = r#"
const C: bool = false;
const D: bool = false;
fn foo() {
    assert!(C && D, "message");
}
"#;

const BOOL_EXPR_SIMPLE_WITH_FUNCTION_CALL: &str = r#"
fn bar(x: felt252) -> bool {
    x == 10
}
fn foo() {
    assert!(true || bar(5), "message");
}
"#;

const BOOL_EXPR_COMPLEX_WITH_FUNCTION_CALL: &str = r#"
fn bar(x: felt252) -> bool {
    x == 10
}
fn foo() {
    assert!(((1 == 1) && (5 == 5)) || bar(5), "message");
}
"#;

/// This is a negative example. Lint should not trigger
/// because the expression we assert on is not constant.
const BOOL_EXPR_NON_CONST_WITH_FUNCTION_CALL: &str = r#"
fn bar(x: felt252) -> bool {
    x == 10
}
fn foo(x: felt252) {
    assert!(bar(x), "message");
}
"#;

const BOOL_CONST_IN_IMPL_FUNCTION: &str = r#"
trait Trait {
    fn foo();
}

impl Impl of Trait {
    fn foo() {
        assert!(true, "message");
    }
}
"#;

const BOOL_CONST_IN_TRAIT_FUNCTION: &str = r#"
trait Trait {
    fn foo() {
        assert!(true, "message");
    }
}
"#;

#[test]
fn bool_literal_diagnostics() {
    test_lint_diagnostics!(BOOL_LITERAL, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:5
        assert!(true, "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_const_diagnostics() {
    test_lint_diagnostics!(BOOL_CONST, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:4:5
        assert!(C, "message");
        ^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_const_negated_diagnostics() {
    test_lint_diagnostics!(BOOL_CONST_NEGATED, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:4:5
        assert!(!C, "message");
        ^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_simple_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_SIMPLE, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:5
        assert!(1 == 1, "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_simple_negated_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_SIMPLE_NEGATED, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:5
        assert!(!((1 == 1) && (2 == 2)), "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_arithmetic_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_ARITHMETIC, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:5
        assert!(1 == (1 + 1), "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_arithmetic_complex_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_ARITHMETIC_COMPLEX, @r"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:3:1
    assert!((1 == 1) && (5 - 5 == 3 - 3));
    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn bool_expr_with_const_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_WITH_CONST, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:5:5
        assert!(C && D, "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_simple_with_function_call_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_SIMPLE_WITH_FUNCTION_CALL, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:6:5
        assert!(true || bar(5), "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_expr_complex_with_function_call_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_COMPLEX_WITH_FUNCTION_CALL, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:6:5
        assert!(((1 == 1) && (5 == 5)) || bar(5), "message");
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn non_const_bool_expr_with_function_call_diagnostics() {
    test_lint_diagnostics!(BOOL_EXPR_NON_CONST_WITH_FUNCTION_CALL, @"")
}

#[test]
fn bool_const_in_impl_function_diagnostics() {
    test_lint_diagnostics!(BOOL_CONST_IN_IMPL_FUNCTION, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:8:9
            assert!(true, "message");
            ^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}

#[test]
fn bool_const_in_trait_function_diagnostics() {
    test_lint_diagnostics!(BOOL_CONST_IN_TRAIT_FUNCTION, @r#"
    Plugin diagnostic: Unnecessary assert on a const value detected.
     --> lib.cairo:4:9
            assert!(true, "message");
            ^^^^^^^^^^^^^^^^^^^^^^^^
    "#)
}
