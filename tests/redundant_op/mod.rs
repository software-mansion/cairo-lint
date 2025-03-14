use super::{test_lint_diagnostics, test_lint_fixer};

const ADDITION_BY_ZERO: &str = r#"
fn main() {
    let x = 42;
    let _y = x + 0;
}
"#;
const SUBTRACTION_BY_ZERO: &str = r#"
fn main() {
    let x = 42;
    let _y = x - 0;
}
"#;
const MULTIPLICATION_BY_ONE: &str = r#"
fn main() {
    let x = 42;
    let _y = x * 1;
}
"#;
const DIVISION_BY_ONE: &str = r#"
fn main() {
    let x = 42_u32;
    let _y = x / 1;
}
"#;

#[test]
fn addition_by_zero_diagnostics() {
    test_lint_diagnostics!(ADDITION_BY_ZERO, @r"
    warning: Plugin diagnostic: This operation doesn't change the value and can be simplified.
     --> lib.cairo:4:14
      |
    4 |     let _y = x + 0;
      |              -----
      |
    ");
}
#[test]
fn addition_by_zero_fixer() {
    test_lint_fixer!(ADDITION_BY_ZERO, @r"
    fn main() {
        let x = 42;
        let _y = x + 0;
    }
    ");
}
#[test]
fn subtraction_by_zero_diagnostics() {
    test_lint_diagnostics!(SUBTRACTION_BY_ZERO, @r"
    warning: Plugin diagnostic: This operation doesn't change the value and can be simplified.
     --> lib.cairo:4:14
      |
    4 |     let _y = x - 0;
      |              -----
      |
    ");
}
#[test]
fn subtraction_by_zero_fixer() {
    test_lint_fixer!(SUBTRACTION_BY_ZERO, @r"
    fn main() {
        let x = 42;
        let _y = x - 0;
    }
    ");
}
#[test]
fn multiplication_by_one_diagnostics() {
    test_lint_diagnostics!(MULTIPLICATION_BY_ONE, @r"
    warning: Plugin diagnostic: This operation doesn't change the value and can be simplified.
     --> lib.cairo:4:14
      |
    4 |     let _y = x * 1;
      |              -----
      |
    ");
}
#[test]
fn multiplication_by_one_fixer() {
    test_lint_fixer!(MULTIPLICATION_BY_ONE, @r#"
    fn main() {
        let x = 42;
        let _y = x * 1;
    }
    "#);
}
#[test]
fn division_by_one_diagnostics() {
    test_lint_diagnostics!(DIVISION_BY_ONE, @r"
    warning: Plugin diagnostic: This operation doesn't change the value and can be simplified.
     --> lib.cairo:4:14
      |
    4 |     let _y = x / 1;
      |              -----
      |
    ");
}
#[test]
fn division_by_one_fixer() {
    test_lint_fixer!(DIVISION_BY_ONE, @r#"
    fn main() {
        let x = 42_u32;
        let _y = x / 1;
    }
    "#);
}
