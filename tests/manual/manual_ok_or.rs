use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_ERROR_STR: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => Result::Ok(v),
        Option::None => Result::Err('this is an err'),
    };
}
"#;

const TEST_ERROR_STR_ALLOWED: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    #[allow(manual_ok_or)]
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => Result::Ok(v),
        Option::None => Result::Err('this is an err'),
    };
}
"#;

const TEST_ERROR_ENUM: &str = r#"
mod Error {
    pub const Error: felt252 = 'this is an err';
}
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => Result::Ok(v),
        Option::None => Result::Err(Error::Error),
    };
}
"#;

const TEST_WITH_COMMENT_IN_NONE: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => Result::Ok(v),
        Option::None =>{
            // do something
            Result::Err('this is an err')
        },
    };
}
"#;

const TEST_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => {
            // do something
            Result::Ok(v)
        },
        Option::None => Result::Err('this is an err'),
    };
}
"#;

const TEST_MATCH_EXPRESSION_NOT_A_VARIABLE: &str = r#"
#[derive(Copy, Drop)]
enum Error {
    Error,
}
fn main() {
    let self: u256 = 0;
    // This is just a variable.
    let _self_result: Result<u8, Error> = match self.try_into() {
        Option::Some(value) => Result::Ok(value),
        Option::None => Result::Err(Error::Error)
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    // This is just a variable.
    let _a = if let Option::Some(val) = opt_val {
        Result::Ok(val)
    } else {
        Result::Err('err')
    };
}
"#;

const TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    // This is just a variable.
    let _a = if let Option::Some(val) = opt_val {
        let val = val + 1;
        Result::Ok(val)
    } else {
        Result::Err('err')
    };
}
"#;

const TEST_OTHER_VAR: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    let other_val = 0;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(_) => Result::Ok(other_val),
        Option::None => Result::Err('this is an err'),
    };
}
"#;

const TEST_IF_OTHER_VAR: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    let other_val = 0;
    // This is just a variable.
    let _a = if let Option::Some(_) = opt_val {
        Result::Ok(other_val)
    } else {
        Result::Err('err')
    };
}
"#;

const TEST_ERROR_STR_BLOCK: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => {
            Result::Ok(v)
        },
        Option::None => {
            Result::Err('this is an err')
        },
    };
}
"#;

const TEST_ERROR_STR_BLOCK_MORE_STATEMENTS: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(v) => {
            Result::Ok(v)
        },
        Option::None => {
            let _a = 23;
            Result::Err('this is an err')
        },
    };
}
"#;

const MATCH_WITH_REVERSED_ARMS: &str = r#"
fn main() {
    let a: Option<usize> = Option::None;
    let _ = match a {
        Option::None => Result::Err('error'),
        Option::Some(v) => Result::Ok(v),
    };
}
"#;

#[test]
fn test_error_str_diagnostics() {
    test_lint_diagnostics!(TEST_ERROR_STR, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:5:16-8:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_error_str_fixer() {
    test_lint_fixer!(TEST_ERROR_STR, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.ok_or('this is an err');
    }
    ");
}

#[test]
fn test_error_str_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_ERROR_STR_ALLOWED, @r#"
    "#);
}

#[test]
fn test_error_str_allowed_fixer() {
    test_lint_fixer!(TEST_ERROR_STR_ALLOWED, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        #[allow(manual_ok_or)]
        // This is just a variable.
        let _foo = match foo {
            Option::Some(v) => Result::Ok(v),
            Option::None => Result::Err('this is an err'),
        };
    }
    ");
}

#[test]
fn test_error_enum_diagnostics() {
    test_lint_diagnostics!(TEST_ERROR_ENUM, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:8:16-11:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_error_enum_fixer() {
    test_lint_fixer!(TEST_ERROR_ENUM, @r"
    mod Error {
        pub const Error: felt252 = 'this is an err';
    }
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.ok_or(Error::Error);
    }
    ");
}

#[test]
fn test_with_comment_in_none_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_NONE, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:5:16-11:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_with_comment_in_none_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_NONE, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.ok_or('this is an err');
    }
    ");
}

#[test]
fn test_with_comment_in_some_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_SOME, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:5:16-11:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_with_comment_in_some_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_SOME, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.ok_or('this is an err');
    }
    ");
}

#[test]
fn test_match_expression_not_a_variable_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_NOT_A_VARIABLE, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:9:43-12:5
          let _self_result: Result<u8, Error> = match self.try_into() {
     ___________________________________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_match_expression_not_a_variable_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_NOT_A_VARIABLE, @r"
    #[derive(Copy, Drop)]
    enum Error {
        Error,
    }
    fn main() {
        let self: u256 = 0;
        // This is just a variable.
        let _self_result: Result<u8, Error> = self.try_into().ok_or(Error::Error);
    }
    ");
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:5:14-9:5
          let _a = if let Option::Some(val) = opt_val {
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
        let opt_val: Option<i32> = Option::None;
        // This is just a variable.
        let _a = opt_val.ok_or('err');
    }
    ");
}

#[test]
fn test_manual_if_with_additional_instructions_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r#"
    "#);
}

#[test]
fn test_manual_if_with_additional_instructions_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        // This is just a variable.
        let _a = if let Option::Some(val) = opt_val {
            let val = val + 1;
            Result::Ok(val)
        } else {
            Result::Err('err')
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
        let foo: Option<i32> = Option::None;
        let other_val = 0;
        // This is just a variable.
        let _foo = match foo {
            Option::Some(_) => Result::Ok(other_val),
            Option::None => Result::Err('this is an err'),
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
        let opt_val: Option<i32> = Option::None;
        let other_val = 0;
        // This is just a variable.
        let _a = if let Option::Some(_) = opt_val {
            Result::Ok(other_val)
        } else {
            Result::Err('err')
        };
    }
    ");
}

#[test]
fn test_error_str_block_diagnostics() {
    test_lint_diagnostics!(TEST_ERROR_STR_BLOCK, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
     --> lib.cairo:5:16-12:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_error_str_block_fixer() {
    test_lint_fixer!(TEST_ERROR_STR_BLOCK, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.ok_or('this is an err');
    }
    ");
}

#[test]
fn test_error_str_block_more_statements_diagnostics() {
    test_lint_diagnostics!(TEST_ERROR_STR_BLOCK_MORE_STATEMENTS, @r"")
}

#[test]
fn test_error_str_block_more_statements_fixer() {
    test_lint_fixer!(TEST_ERROR_STR_BLOCK_MORE_STATEMENTS, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = match foo {
            Option::Some(v) => { Result::Ok(v) },
            Option::None => {
                let _a = 23;
                Result::Err('this is an err')
            },
        };
    }
    ")
}

#[test]
fn match_with_reversed_arms_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_REVERSED_ARMS, @r"
    Plugin diagnostic: Manual match for Option<T> detected. Consider using ok_or instead
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
        let a: Option<usize> = Option::None;
        let _ = a.ok_or('error');
    }
    ");
}
