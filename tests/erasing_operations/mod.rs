use crate::{test_lint_diagnostics, test_lint_fixer};

const MULTIPLICATION_BY_ZERO: &str = r#"
fn main() {
    let x = 1;
    let _y = 0 * x;
    let _z = x * 0;
}
"#;

const DIVISION_BY_ZERO: &str = r#"
fn main() {
    let x = 1_u32;
    let _y = 0 / x;
}
"#;

const DIVISION_BY_ZERO_ALLOWED: &str = r#"
fn main() {
    let x = 1_u32;
    #[allow(erasing_op)]
    let _y = 0 / x;
}
"#;

const BITWISE_AND_WITH_ZERO: &str = r#"
fn main() {
    let x = 1_u32;
    let _y = x & 0;
    let _z = 0 & x;
}
"#;

const MULTIPLE_OPERATIONS: &str = r#"
fn main() {
    let x = 1_u32;
    let y = 5_u32;
    let z = 10_u32;
    let _f = ((x + y) * 0) & (z / 2);
}
"#;

const MULTIPLE_BITWISE_OPERATIONS: &str = r#"
fn main() {
    let x = 1_u32;
    let y = 5_u32;
    let z = 10_u32;
    let _result1 = (x * y + z) & (z & 0) ^ (z - y);
}
"#;

#[test]
fn multiplication_by_zero_diagnostics() {
    test_lint_diagnostics!(MULTIPLICATION_BY_ZERO, @r"
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:4:14
        let _y = 0 * x;
                 ^^^^^
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:5:14
        let _z = x * 0;
                 ^^^^^
    ");
}

#[test]
fn multiplication_by_zero_fixer() {
    test_lint_fixer!(MULTIPLICATION_BY_ZERO, @r#"
    fn main() {
        let x = 1;
        let _y = 0 * x;
        let _z = x * 0;
    }
    "#);
}

#[test]
fn division_by_zero_diagnostics() {
    test_lint_diagnostics!(DIVISION_BY_ZERO, @r"
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:4:14
        let _y = 0 / x;
                 ^^^^^
    ");
}

#[test]
fn division_by_zero_fixer() {
    test_lint_fixer!(DIVISION_BY_ZERO, @r#"
    fn main() {
        let x = 1_u32;
        let _y = 0 / x;
    }
    "#);
}

#[test]
fn division_by_zero_allowed_diagnostics() {
    test_lint_diagnostics!(DIVISION_BY_ZERO_ALLOWED, @r#"
    "#);
}

#[test]
fn division_by_zero_allowed_fixer() {
    test_lint_fixer!(DIVISION_BY_ZERO_ALLOWED, @r#"
    fn main() {
        let x = 1_u32;
        #[allow(erasing_op)]
        let _y = 0 / x;
    }
    "#);
}

#[test]
fn bitwise_and_with_zero_diagnostics() {
    test_lint_diagnostics!(BITWISE_AND_WITH_ZERO, @r"
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:4:14
        let _y = x & 0;
                 ^^^^^
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:5:14
        let _z = 0 & x;
                 ^^^^^
    ");
}

#[test]
fn bitwise_and_with_zero_fixer() {
    test_lint_fixer!(BITWISE_AND_WITH_ZERO, @r#"
    fn main() {
        let x = 1_u32;
        let _y = x & 0;
        let _z = 0 & x;
    }
    "#);
}

#[test]
fn multiple_operations_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_OPERATIONS, @r"
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:6:15
        let _f = ((x + y) * 0) & (z / 2);
                  ^^^^^^^^^^^
    ");
}

#[test]
fn multiple_operations_fixer() {
    test_lint_fixer!(MULTIPLE_OPERATIONS, @r"
    fn main() {
        let x = 1_u32;
        let y = 5_u32;
        let z = 10_u32;
        let _f = ((x + y) * 0) & (z / 2);
    }
    ");
}

#[test]
fn multiple_bitwise_operations_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_BITWISE_OPERATIONS, @r"
    Plugin diagnostic: This operation results in the value being erased (e.g., multiplication by 0). Consider replacing the entire expression with 0.
     --> lib.cairo:6:35
        let _result1 = (x * y + z) & (z & 0) ^ (z - y);
                                      ^^^^^
    ");
}

#[test]
fn multiple_bitwise_operations_fixer() {
    test_lint_fixer!(MULTIPLE_BITWISE_OPERATIONS, @r#"
    fn main() {
        let x = 1_u32;
        let y = 5_u32;
        let z = 10_u32;
        let _result1 = (x * y + z) & (z & 0) ^ (z - y);
    }
    "#);
}
