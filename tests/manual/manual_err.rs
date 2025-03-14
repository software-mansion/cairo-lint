use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => Option::None,
        Result::Err(x) => Option::Some(x),
    };
}
"#;

const TEST_BASIC_ERR_ALLOWED: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    #[allow(manual_err)]
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => Option::None,
        Result::Err(x) => Option::Some(x),
    };
}
"#;

const TEST_BASIC_IF_ERR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = if let Result::Err(x) = res_val {
        Option::Some(x)
    } else {
        Option::None
    };
}
"#;

const TEST_IF_OTHER_ERR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let other_err = 'err';
    // This is just a variable.
    let _a = if let Result::Err(_) = res_val {
        Option::Some(other_err)
    } else {
        Option::None
    };
}
"#;

const TEST_OTHER_ERR: &str = r#"
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let other_err = 'other err';
    // This is just a variable.
    let _foo = match foo {
        Result::Ok(_) => Option::None,
        Result::Err(_) => Option::Some(other_err),
    };
}
"#;

#[test]
fn test_basic_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_ERR, @r"
    warning: Plugin diagnostic: Manual match for `err` detected. Consider using `err()` instead
     --> lib.cairo:5:16
      |
    5 |       let _foo = match foo {
      |  ________________-
    6 | |         Result::Ok(_) => Option::None,
    7 | |         Result::Err(x) => Option::Some(x),
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_err_fixer() {
    test_lint_fixer!(TEST_BASIC_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _foo = foo.err();
    }
    ");
}

#[test]
fn test_basic_err_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_ERR_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_err_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_ERR_ALLOWED, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        #[allow(manual_err)]
        // This is just a variable.
        let _foo = match foo {
            Result::Ok(_) => Option::None,
            Result::Err(x) => Option::Some(x),
        };
    }
    ");
}

#[test]
fn test_basic_if_err_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IF_ERR, @r"
    warning: Plugin diagnostic: Manual match for `err` detected. Consider using `err()` instead
     --> lib.cairo:5:14
      |
    5 |       let _a = if let Result::Err(x) = res_val {
      |  ______________-
    6 | |         Option::Some(x)
    7 | |     } else {
    8 | |         Option::None
    9 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_if_err_fixer() {
    test_lint_fixer!(TEST_BASIC_IF_ERR, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.err();
    }
    ");
}

#[test]
fn test_if_other_err_diagnostics() {
    test_lint_diagnostics!(TEST_IF_OTHER_ERR, @r#"
    "#);
}

#[test]
fn test_if_other_err_fixer() {
    test_lint_fixer!(TEST_IF_OTHER_ERR, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let other_err = 'err';
        // This is just a variable.
        let _a = if let Result::Err(_) = res_val {
            Option::Some(other_err)
        } else {
            Option::None
        };
    }
    ");
}

#[test]
fn test_other_err_diagnostics() {
    test_lint_diagnostics!(TEST_OTHER_ERR, @r#"
    "#);
}

#[test]
fn test_other_err_fixer() {
    test_lint_fixer!(TEST_OTHER_ERR, @r"
    fn main() {
        let foo: Result<i32> = Result::Err('err');
        let other_err = 'other err';
        // This is just a variable.
        let _foo = match foo {
            Result::Ok(_) => Option::None,
            Result::Err(_) => Option::Some(other_err),
        };
    }
    ");
}
