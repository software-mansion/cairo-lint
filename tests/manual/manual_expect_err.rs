use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_MATCH_EXPECT_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let err = 'this is an err';
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => core::panic_with_felt252(err),
        Result::Err(x) => x,
    };
}
"#;

const TEST_BASIC_MATCH_EXPECT_ERR_ALLOWED: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let err = 'this is an err';
    #[allow(manual_expect_err)]
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => core::panic_with_felt252(err),
        Result::Err(x) => x,
    };
}
"#;

const TEST_BASIC_IF_EXPECT_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = if let Result::Err(err) = foo {
        err
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

const TEST_MATCH_WITH_OTHER_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let other_err = 'this is an err';
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => core::panic_with_felt252('error'),
        Result::Err(_) => other_err,
    };
}
"#;

const TEST_IF_WITH_OTHER_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let other_err = 'other err';
    // This is just a variable.
    let _a = if let Result::Err(_) = foo {
        other_err
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

const TEST_MATCH_WITH_FUNCTION: &str = r#"
fn foo(x : i32) -> Result<i32, felt252> {
    Result::Ok('i32')
} 
fn main() {
    // This is just a variable.
    let _foo = match foo(0) {
        Result::Ok(_) => core::panic_with_felt252('error'),
        Result::Err(err) => err,
    };
}
"#;

const TEST_IF_WITH_FUNCTION: &str = r#"
fn foo(x : i32) -> Result<i32, felt252> {
    Result::Ok('i32')
} 
fn main() {
    // This is just a variable.
    let _a = if let Result::Err(err) = foo(0) {
        err
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

#[test]
fn test_basic_match_expect_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MATCH_EXPECT_ERR, @r"
    warning: Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:6:16
      |
    6 |       let _foo = match foo {
      |  ________________-
    7 | |         Result::Ok(_) => core::panic_with_felt252(err),
    8 | |         Result::Err(x) => x,
    9 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_match_expect_err_fixer() {
    test_lint_fixer!(TEST_BASIC_MATCH_EXPECT_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let err = 'this is an err';
        // This is just a variable.
        let _foo = foo.expect_err(err);
    }
    ");
}

#[test]
fn test_basic_match_expect_err_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MATCH_EXPECT_ERR_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_match_expect_err_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_MATCH_EXPECT_ERR_ALLOWED, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let err = 'this is an err';
        #[allow(manual_expect_err)]
        // This is just a variable.
        let _foo = match foo {
            Result::Ok(_) => core::panic_with_felt252(err),
            Result::Err(x) => x,
        };
    }
    ");
}

#[test]
fn test_basic_if_expect_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IF_EXPECT_ERR, @r"
    warning: Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:5:14
      |
    5 |       let _a = if let Result::Err(err) = foo {
      |  ______________-
    6 | |         err
    7 | |     } else {
    8 | |         core::panic_with_felt252('panic')
    9 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_if_expect_err_fixer() {
    test_lint_fixer!(TEST_BASIC_IF_EXPECT_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = foo.expect_err('panic');
    }
    ");
}

#[test]
fn test_match_with_other_err_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_WITH_OTHER_ERR, @r#"
    "#);
}

#[test]
fn test_match_with_other_err_fixer() {
    test_lint_fixer!(TEST_MATCH_WITH_OTHER_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let other_err = 'this is an err';
        // This is just a variable.
        let _foo = match foo {
            Result::Ok(_) => core::panic_with_felt252('error'),
            Result::Err(_) => other_err,
        };
    }
    ");
}

#[test]
fn test_if_with_other_err_diagnostics() {
    test_lint_diagnostics!(TEST_IF_WITH_OTHER_ERR, @r#"
    "#);
}

#[test]
fn test_if_with_other_err_fixer() {
    test_lint_fixer!(TEST_IF_WITH_OTHER_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let other_err = 'other err';
        // This is just a variable.
        let _a = if let Result::Err(_) = foo {
            other_err
        } else {
            core::panic_with_felt252('panic')
        };
    }
    ");
}

#[test]
fn test_match_with_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_WITH_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
      --> lib.cairo:7:16
       |
     7 |       let _foo = match foo(0) {
       |  ________________-
     8 | |         Result::Ok(_) => core::panic_with_felt252('error'),
     9 | |         Result::Err(err) => err,
    10 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_match_with_function_fixer() {
    test_lint_fixer!(TEST_MATCH_WITH_FUNCTION, @r"
    fn foo(x : i32) -> Result<i32, felt252> {
        Result::Ok('i32')
    } 
    fn main() {
        // This is just a variable.
        let _foo = foo(0).expect_err('error');
    }
    ");
}

#[test]
fn test_if_with_function_diagnostics() {
    test_lint_diagnostics!(TEST_IF_WITH_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
      --> lib.cairo:7:14
       |
     7 |       let _a = if let Result::Err(err) = foo(0) {
       |  ______________-
     8 | |         err
     9 | |     } else {
    10 | |         core::panic_with_felt252('panic')
    11 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_if_with_function_fixer() {
    test_lint_fixer!(TEST_IF_WITH_FUNCTION, @r"
    fn foo(x : i32) -> Result<i32, felt252> {
        Result::Ok('i32')
    } 
    fn main() {
        // This is just a variable.
        let _a = foo(0).expect_err('panic');
    }
    ");
}
