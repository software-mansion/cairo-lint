use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_ERR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
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
    // This is just a variable.
    let _a = match foo(0) {
        Result::Ok(_) => false,
        Result::Err(_) => true
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
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
    // This is just a variable.
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
    // This is just a variable.
    let _a = if let Result::Ok(_) = foo(0) {
        false
    } else {
        true
    };
}
"#;

const TEST_BASIC_IS_ERR_BLOCK: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = match res_val {
        Result::Ok(_) => {
            false
        },
        Result::Err(_) => {
            true
        },
    };
}
"#;

const MATCH_WITH_REVERSED_ARMS: &str = r#"
fn main() {
    let a: Result<usize, ()> = Result::Err(());
    let _ = match a {
        Result::Err(_) => true,
        Result::Ok(_) => false,
    };
}
"#;

#[test]
fn test_basic_is_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_ERR, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:5:14-8:5
          let _a = match res_val {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_is_err_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_ERR, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.is_err();
    }
    ");
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:7:14-10:5
          let _a = match foo(0) {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_match_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    fn foo(a: i32) -> Result<i32, felt252> {
        Result::Err('err')
    }
    fn main() {
        // This is just a variable.
        let _a = foo(0).is_err();
    }
    ");
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:5:14-9:5
          let _a = if let Result::Ok(_) = res_val {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.is_err();
    }
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:7:14-11:5
          let _a = if let Result::Ok(_) = foo(0) {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION, @r"
    fn foo(a: i32) -> Result<i32, felt252> {
        Result::Err('err')
    }
    fn main() {
        // This is just a variable.
        let _a = foo(0).is_err();
    }
    ");
}

#[test]
fn test_manual_if_expression_is_a_function_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION_ALLOWED, @r#"
    "#);
}

#[test]
fn test_manual_if_expression_is_a_function_allowed_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_EXPRESSION_IS_A_FUNCTION_ALLOWED, @r"
    fn foo(a: i32) -> Result<i32, felt252> {
        Result::Err('err')
    }
    fn main() {
        #[allow(manual_is_err)]
        // This is just a variable.
        let _a = if let Result::Ok(_) = foo(0) {
            false
        } else {
            true
        };
    }
    ");
}

#[test]
fn test_basic_is_err_block_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_ERR_BLOCK, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:5:14-12:5
          let _a = match res_val {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_is_err_block_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_ERR_BLOCK, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.is_err();
    }
    ");
}

#[test]
fn match_with_reversed_arms_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_REVERSED_ARMS, @r"
    Plugin diagnostic: Manual match for `is_err` detected. Consider using `is_err()` instead
     --> lib.cairo:4:13-7:5
          let _ = match a {
     _____________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn match_with_reversed_arms_fixer() {
    test_lint_fixer!(MATCH_WITH_REVERSED_ARMS, @r"
    fn main() {
        let a: Result<usize, ()> = Result::Err(());
        let _ = a.is_err();
    }
    ");
}
