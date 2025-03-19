use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_OK: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = match res_val {
        Result::Ok(x) => Option::Some(x),
        Result::Err(_) => Option::None,
    };
}
"#;

const TEST_BASIC_IF_OK: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    // This is just a variable.
    let _a = if let Result::Ok(x) = res_val {
        Option::Some(x)
    } else {
        Option::None
    };
}
"#;

const TEST_BASIC_IF_OK_ALLOWED: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    #[allow(manual_ok)]
    // This is just a variable.
    let _a = if let Result::Ok(x) = res_val {
        Option::Some(x)
    } else {
        Option::None
    };
}
"#;

const TEST_OTHER_VAR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let other_val = 0;
    // This is just a variable.
    let _a = match res_val {
        Result::Ok(_) => Option::Some(other_val),
        Result::Err(_) => Option::None,
    };
}
"#;

const TEST_IF_OTHER_VAR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let other_val = 0;
    // This is just a variable.
    let _a = if let Result::Ok(_) = res_val {
        Option::Some(other_val)
    } else {
        Option::None
    };
}
"#;

#[test]
fn test_basic_ok_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_OK, @r"
    Plugin diagnostic: Manual match for `ok` detected. Consider using `ok()` instead
     --> lib.cairo:5:14-8:5
          let _a = match res_val {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_ok_fixer() {
    test_lint_fixer!(TEST_BASIC_OK, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.ok();
    }
    ");
}

#[test]
fn test_basic_if_ok_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IF_OK, @r"
    Plugin diagnostic: Manual match for `ok` detected. Consider using `ok()` instead
     --> lib.cairo:5:14-9:5
          let _a = if let Result::Ok(x) = res_val {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_if_ok_fixer() {
    test_lint_fixer!(TEST_BASIC_IF_OK, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        // This is just a variable.
        let _a = res_val.ok();
    }
    ");
}

#[test]
fn test_basic_if_ok_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IF_OK_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_if_ok_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IF_OK_ALLOWED, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        #[allow(manual_ok)]
        // This is just a variable.
        let _a = if let Result::Ok(x) = res_val {
            Option::Some(x)
        } else {
            Option::None
        };
    }
    ");
}

#[test]
fn test_other_var_diagnostics() {
    test_lint_diagnostics!(TEST_OTHER_VAR, @r#"
    "#);
}

#[test]
fn test_other_var_fixer() {
    test_lint_fixer!(TEST_OTHER_VAR, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let other_val = 0;
        // This is just a variable.
        let _a = match res_val {
            Result::Ok(_) => Option::Some(other_val),
            Result::Err(_) => Option::None,
        };
    }
    ");
}

#[test]
fn test_if_other_var_diagnostics() {
    test_lint_diagnostics!(TEST_IF_OTHER_VAR, @r#"
    "#);
}

#[test]
fn test_if_other_var_fixer() {
    test_lint_fixer!(TEST_IF_OTHER_VAR, @r"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let other_val = 0;
        // This is just a variable.
        let _a = if let Result::Ok(_) = res_val {
            Option::Some(other_val)
        } else {
            Option::None
        };
    }
    ");
}
