use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_OK: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = match res_val {
        Result::Ok(_) => true,
        Result::Err(_) => false
    };
}
"#;

const TEST_BASIC_IS_OK_ALLOWED: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    #[allow(manual_is_ok)]
    // This is just a variable.
    let _a = match res_val {
        Result::Ok(_) => true,
        Result::Err(_) => false
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = if let Result::Ok(_) = res_val {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION: &str = r#"
fn foo(a: i32) -> Result<i32,felt252> {
    Result::Err('err')
}
fn main() {
    // This is just a variable.
    let _a = if let Result::Ok(_) = foo(0) {
        true
    } else {
        false
    };
}
"#;

const TEST_MATCH_EXPRESSION_IS_A_FUNCTION: &str = r#"
fn foo(a: i32) -> Result<i32,felt252> {
    Result::Err('err')
}
fn main() {
    // This is just a variable.
    let _a = match foo(0) {
        Result::Ok(_) => true,
        Result::Err(_) => false
    };
}
"#;

#[test]
fn test_basic_is_ok_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_OK, @r"
    warning: Plugin diagnostic: Manual match for `is_ok` detected. Consider using `is_ok()` instead
     --> lib.cairo:5:14
      |
    5 |       let _a = match res_val {
      |  ______________-
    6 | |         Result::Ok(_) => true,
    7 | |         Result::Err(_) => false
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_is_ok_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_OK, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.is_ok();
    }
    ");
}

#[test]
fn test_basic_is_ok_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_OK_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_is_ok_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_OK_ALLOWED, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        #[allow(manual_is_ok)]
        // This is just a variable.
        let _a = match res_val {
            Result::Ok(_) => true,
            Result::Err(_) => false
        };
    }
    ");
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    warning: Plugin diagnostic: Manual match for `is_ok` detected. Consider using `is_ok()` instead
     --> lib.cairo:5:14
      |
    5 |       let _a = if let Result::Ok(_) = res_val {
      |  ______________-
    6 | |         true
    7 | |     } else {
    8 | |         false
    9 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.is_ok();
    }
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_ok` detected. Consider using `is_ok()` instead
      --> lib.cairo:7:14
       |
     7 |       let _a = if let Result::Ok(_) = foo(0) {
       |  ______________-
     8 | |         true
     9 | |     } else {
    10 | |         false
    11 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r"
    fn foo(a: i32) -> Result<i32,felt252> {
        Result::Err('err')
    }
    fn main() {
        // This is just a variable.
        let _a = foo(0).is_ok();
    }
    ");
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_ok` detected. Consider using `is_ok()` instead
      --> lib.cairo:7:14
       |
     7 |       let _a = match foo(0) {
       |  ______________-
     8 | |         Result::Ok(_) => true,
     9 | |         Result::Err(_) => false
    10 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_match_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    fn foo(a: i32) -> Result<i32,felt252> {
        Result::Err('err')
    }
    fn main() {
        // This is just a variable.
        let _a = foo(0).is_ok();
    }
    ");
}
