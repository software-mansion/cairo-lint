use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_EQ_OP: &str = r#"
fn foo(a: u256) -> bool {
    a == a
}
"#;

const SIMPLE_NEQ_OP: &str = r#"
fn foo(a: u256) -> bool {
    a != a
}
"#;

const SIMPLE_LT_OP: &str = r#"
fn foo(a: u256) -> bool {
    a < a
}
"#;

const SIMPLE_GT_OP: &str = r#"
fn foo(a: u256) -> bool {
    a > a
}
"#;

const SIMPLE_BITWISE_OP: &str = r#"
fn foo(a: u256) -> u256 {
    a & a
}
"#;

const SIMPLE_BITWISE_OP_ALLOWED: &str = r#"
fn foo(a: u256) -> u256 {
    #[allow(eq_logical_op)]
    a & a
}
"#;

const SIMPLE_SUB_OP: &str = r#"
fn foo(a: u256) -> u256 {
    a - a
}
"#;

const SIMPLE_DIVIDE_OP: &str = r#"
fn foo(a: u256) -> u256 {
    a / a
}
"#;

const OP_WITH_METHOD_CALL: &str = r#"
fn foo(a: Array<u256>) -> bool {
    a.len() == a.len()
}
"#;

#[test]
fn simple_eq_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_EQ_OP, @r"
    Plugin diagnostic: Comparison with identical operands, this operation always results in true and may indicate a logic error
     --> lib.cairo:3:5
        a == a
        ^^^^^^
    ");
}

#[test]
fn simple_eq_op_fixer() {
    test_lint_fixer!(SIMPLE_EQ_OP, @r#"
    fn foo(a: u256) -> bool {
        a == a
    }
    "#);
}

#[test]
fn simple_neq_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_NEQ_OP, @r"
    Plugin diagnostic: Comparison with identical operands, this operation always results in false and may indicate a logic error
     --> lib.cairo:3:5
        a != a
        ^^^^^^
    ");
}

#[test]
fn simple_neq_op_fixer() {
    test_lint_fixer!(SIMPLE_NEQ_OP, @r#"
    fn foo(a: u256) -> bool {
        a != a
    }
    "#);
}

#[test]
fn simple_lt_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LT_OP, @r"
    Plugin diagnostic: Comparison with identical operands, this operation always results in false and may indicate a logic error
     --> lib.cairo:3:5
        a < a
        ^^^^^
    ");
}

#[test]
fn simple_lt_op_fixer() {
    test_lint_fixer!(SIMPLE_LT_OP, @r#"
    fn foo(a: u256) -> bool {
        a < a
    }
    "#);
}

#[test]
fn simple_gt_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_GT_OP, @r"
    Plugin diagnostic: Comparison with identical operands, this operation always results in false and may indicate a logic error
     --> lib.cairo:3:5
        a > a
        ^^^^^
    ");
}

#[test]
fn simple_gt_op_fixer() {
    test_lint_fixer!(SIMPLE_GT_OP, @r#"
    fn foo(a: u256) -> bool {
        a > a
    }
    "#);
}

#[test]
fn simple_bitwise_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_BITWISE_OP, @r"
    Plugin diagnostic: Logical operation with identical operands, this operation always results in the same value and may indicate a logic error
     --> lib.cairo:3:5
        a & a
        ^^^^^
    ");
}

#[test]
fn simple_bitwise_op_fixer() {
    test_lint_fixer!(SIMPLE_BITWISE_OP, @r#"
    fn foo(a: u256) -> u256 {
        a & a
    }
    "#);
}

#[test]
fn simple_bitwise_op_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_BITWISE_OP_ALLOWED, @r#"
    "#);
}

#[test]
fn simple_bitwise_op_allowed_fixer() {
    test_lint_fixer!(SIMPLE_BITWISE_OP_ALLOWED, @r#"
    fn foo(a: u256) -> u256 {
        #[allow(eq_logical_op)]
        a & a
    }
    "#);
}

#[test]
fn simple_sub_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_SUB_OP, @r"
    Plugin diagnostic: Subtraction with identical operands, this operation always results in zero and may indicate a logic error
     --> lib.cairo:3:5
        a - a
        ^^^^^
    ");
}

#[test]
fn simple_sub_op_fixer() {
    test_lint_fixer!(SIMPLE_SUB_OP, @r#"
    fn foo(a: u256) -> u256 {
        a - a
    }
    "#);
}

#[test]
fn simple_divide_op_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DIVIDE_OP, @r"
    Plugin diagnostic: Division with identical operands, this operation always results in one (except for zero) and may indicate a logic error
     --> lib.cairo:3:5
        a / a
        ^^^^^
    ");
}

#[test]
fn simple_divide_op_fixer() {
    test_lint_fixer!(SIMPLE_DIVIDE_OP, @r#"
    fn foo(a: u256) -> u256 {
        a / a
    }
    "#);
}

#[test]
fn op_with_method_call_diagnostics() {
    test_lint_diagnostics!(OP_WITH_METHOD_CALL, @r#"
    "#);
}

#[test]
fn op_with_method_call_fixer() {
    test_lint_fixer!(OP_WITH_METHOD_CALL, @r#"
    fn foo(a: Array<u256>) -> bool {
        a.len() == a.len()
    }
    "#);
}
