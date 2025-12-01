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

const TEST_BASIC_MATCH_EXPECT_ERR_BLOCK: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let err = 'this is an err';
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => {
            core::panic_with_felt252(err)
        },
        Result::Err(x) => {
            x
        },
    };
}
"#;

const TEST_MATCH_WITH_REVERSED_ARMS: &str = r#"
fn main() {
    let a: Result<usize> = Result::Err('error');
    let _ = match a {
        Result::Err(err) => err,
        Result::Ok(_) => core::panic_with_felt252('other-error'),
    };
}
"#;

#[test]
fn test_basic_match_expect_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MATCH_EXPECT_ERR, @r"
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:6:16-9:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
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
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:5:14-9:5
          let _a = if let Result::Err(err) = foo {
     ______________^
    | ...
    |     };
    |_____^
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
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:7:16-10:5
          let _foo = match foo(0) {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_match_with_function_fixer() {
    test_lint_fixer!(TEST_MATCH_WITH_FUNCTION, @r"
    fn foo(x: i32) -> Result<i32, felt252> {
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
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:7:14-11:5
          let _a = if let Result::Err(err) = foo(0) {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_if_with_function_fixer() {
    test_lint_fixer!(TEST_IF_WITH_FUNCTION, @r"
    fn foo(x: i32) -> Result<i32, felt252> {
        Result::Ok('i32')
    }
    fn main() {
        // This is just a variable.
        let _a = foo(0).expect_err('panic');
    }
    ");
}

#[test]
fn test_basic_match_expect_err_block_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MATCH_EXPECT_ERR_BLOCK, @r"
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
     --> lib.cairo:6:16-13:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_match_expect_err_block_fixer() {
    test_lint_fixer!(TEST_BASIC_MATCH_EXPECT_ERR_BLOCK, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let err = 'this is an err';
        // This is just a variable.
        let _foo = foo.expect_err(err);
    }
    ");
}

#[test]
fn match_with_reversed_arms_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_WITH_REVERSED_ARMS, @r"
    Plugin diagnostic: Manual match for `expect_err` detected. Consider using `expect_err()` instead
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
    test_lint_fixer!(TEST_MATCH_WITH_REVERSED_ARMS, @r"
    fn main() {
        let a: Result<usize> = Result::Err('error');
        let _ = a.expect_err('other-error');
    }
    ");
}
