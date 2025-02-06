use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_ERR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(_) => false,
        Result::Err(_) => true
    };
}
"#;

const TEST_MATCH_EXPRESSION_IS_A_FUNCTION: &str = r#"
fn foo(a: i32) -> Result<i32,felt252> {
    Result::Err('err')
}
fn main() {
    let _a = match foo(0) {
        Result::Ok(_) => false,
        Result::Err(_) => true
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = if let Result::Ok(_) = res_val {
        false
    } else {
        true
    };
}
"#;

const TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION: &str = r#"
fn foo(a: i32) -> Result<i32,felt252> {
    Result::Err('err')
}
fn main() {
    let _a = if let Result::Ok(_) = foo(0) {
        false
    } else {
        true
    };
}
"#;

const TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION_ALLOWED: &str = r#"
fn foo(a: i32) -> Result<i32,felt252> {
    Result::Err('err')
}
fn main() {
    #[allow(manual_is_err)]
    let _a = if let Result::Ok(_) = foo(0) {
        false
    } else {
        true
    };
}
"#;

#[test]
fn test_basic_is_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_ERR, @r"
    warning: Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = match res_val {
      |  ______________-
    5 | |         Result::Ok(_) => false,
    6 | |         Result::Err(_) => true
    7 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_is_err_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_ERR, @r#"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let _a = res_val.is_err();
    }
    "#);
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:6:14
      |
    6 |       let _a = match foo(0) {
      |  ______________-
    7 | |         Result::Ok(_) => false,
    8 | |         Result::Err(_) => true
    9 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_match_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r#"
    fn foo(a: i32) -> Result<i32,felt252> {
        Result::Err('err')
    }
    fn main() {
        let _a = foo(0).is_err();
    }
    "#);
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    warning: Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = if let Result::Ok(_) = res_val {
      |  ______________-
    5 | |         false
    6 | |     } else {
    7 | |         true
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r#"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let _a = res_val.is_err();
    }
    "#);
}

#[test]
fn test_manual_if_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
      --> lib.cairo:6:14
       |
     6 |       let _a = if let Result::Ok(_) = foo(0) {
       |  ______________-
     7 | |         false
     8 | |     } else {
     9 | |         true
    10 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r#"
    fn foo(a: i32) -> Result<i32,felt252> {
        Result::Err('err')
    }
    fn main() {
        let _a = foo(0).is_err();
    }
    "#);
}

#[test]
fn test_manual_if_expression_is_a_function_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION_ALLOWED, @r#"
    "#);
}

#[test]
fn test_manual_if_expression_is_a_function_allowed_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION_ALLOWED, @r#"
    fn foo(a: i32) -> Result<i32,felt252> {
        Result::Err('err')
    }
    fn main() {
        #[allow(manual_is_err)]
        let _a = if let Result::Ok(_) = foo(0) {
            false
        } else {
            true
        };
    }
    "#);
}
