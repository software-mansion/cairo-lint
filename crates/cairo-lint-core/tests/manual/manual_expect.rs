use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_CORE_PANIC_WITH_FELT252: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => x,
        Option::None => core::panic_with_felt252('err'),
    };
}
"#;

const TEST_PANIC_WITH_FELT252: &str = r#"
use core::panic_with_felt252;
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => x,
        Option::None => panic_with_felt252('err'),
    };
}
"#;

const TEST_WITH_ENUM_ERROR: &str = r#"
mod Error {
    pub const Error: felt252 = 'this is an err';
}
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => x,
        Option::None => core::panic_with_felt252(Error::Error),
    };
}
"#;

const TEST_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => {
            // do something
            x
        },
        Option::None => core::panic_with_felt252('err'),
    };
}
"#;

const TEST_WITH_COMMENT_IN_NONE: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => x,
        Option::None => 
        {
            // do something
            core::panic_with_felt252('err')
        },
    };
}
"#;

const TEST_MATCH_EXPRESSION_IS_A_FUNCTION: &str = r#"
fn foo(a: u256) -> Option<u256> {
    Option::Some(a)
} 
fn main() {
    let a: u256 = 0; 
    let _a = match foo(a) {
        Option::Some(value) => value,
        Option::None => core::panic_with_felt252('err')
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    let _a = if let Option::Some(val) = opt_val {
        val
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

const TEST_MANUAL_IF_ALLOWED: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    #[allow(manual_expect)]
    let _a = if let Option::Some(val) = opt_val {
        val
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

const TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    let _a = if let Option::Some(val) = opt_val {
        let val = val + 1;
        val
    } else {
        core::panic_with_felt252('panic')
    };
}
"#;

const TEST_MANUAL_RESULT_IF: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = if let Result::Ok(x) = res_val {
        x
    } else {
        core::panic_with_felt252('err')
    };
}
"#;

const TEST_MANUAL_MATCH_RESULT: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(val) => val,
        Result::Err(_) => core::panic_with_felt252('error')
    };
}
"#;

const TEST_MANUAL_MATCH_RESULT_WITH_UNWRAPPED_ERROR: &str = r#"
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(val) => val,
        Result::Err(err) => core::panic_with_felt252(err)
    };
}
"#;

#[test]
fn test_core_panic_with_felt252_diagnostics() {
    test_lint_diagnostics!(TEST_CORE_PANIC_WITH_FELT252, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:4:16
      |
    4 |       let _foo = match foo {
      |  ________________-
    5 | |         Option::Some(x) => x,
    6 | |         Option::None => core::panic_with_felt252('err'),
    7 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_core_panic_with_felt252_fixer() {
    test_lint_fixer!(TEST_CORE_PANIC_WITH_FELT252, @r#"
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = foo.expect('err');
    }
    "#);
}

#[test]
fn test_panic_with_felt252_diagnostics() {
    test_lint_diagnostics!(TEST_PANIC_WITH_FELT252, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:5:16
      |
    5 |       let _foo = match foo {
      |  ________________-
    6 | |         Option::Some(x) => x,
    7 | |         Option::None => panic_with_felt252('err'),
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_panic_with_felt252_fixer() {
    test_lint_fixer!(TEST_PANIC_WITH_FELT252, @r#"
    use core::panic_with_felt252;
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = foo.expect('err');
    }
    "#);
}

#[test]
fn test_with_enum_error_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_ENUM_ERROR, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
      --> lib.cairo:7:16
       |
     7 |       let _foo = match foo {
       |  ________________-
     8 | |         Option::Some(x) => x,
     9 | |         Option::None => core::panic_with_felt252(Error::Error),
    10 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_with_enum_error_fixer() {
    test_lint_fixer!(TEST_WITH_ENUM_ERROR, @r#"
    mod Error {
        pub const Error: felt252 = 'this is an err';
    }
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = foo.expect(Error::Error);
    }
    "#);
}

#[test]
fn test_with_comment_in_some_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_SOME, @r#"
    "#);
}

#[test]
fn test_with_comment_in_some_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_SOME, @r#"
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = match foo {
            Option::Some(x) => {
                // do something
                x
            },
            Option::None => core::panic_with_felt252('err'),
        };
    }
    "#);
}

#[test]
fn test_with_comment_in_none_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_NONE, @r#"
    "#);
}

#[test]
fn test_with_comment_in_none_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_NONE, @r#"
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = match foo {
            Option::Some(x) => x,
            Option::None => 
            {
                // do something
                core::panic_with_felt252('err')
            },
        };
    }
    "#);
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
      --> lib.cairo:7:14
       |
     7 |       let _a = match foo(a) {
       |  ______________-
     8 | |         Option::Some(value) => value,
     9 | |         Option::None => core::panic_with_felt252('err')
    10 | |     };
       | |_____-
       |
    ");
}

#[test]
fn test_match_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r#"
    fn foo(a: u256) -> Option<u256> {
        Option::Some(a)
    } 
    fn main() {
        let a: u256 = 0; 
        let _a = foo(a).expect('err');
    }
    "#);
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = if let Option::Some(val) = opt_val {
      |  ______________-
    5 | |         val
    6 | |     } else {
    7 | |         core::panic_with_felt252('panic')
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r#"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        let _a = opt_val.expect('panic');
    }
    "#);
}

#[test]
fn test_manual_if_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_ALLOWED, @r#"
    "#);
}

#[test]
fn test_manual_if_allowed_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_ALLOWED, @r#"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        #[allow(manual_expect)]
        let _a = if let Option::Some(val) = opt_val {
            val
        } else {
            core::panic_with_felt252('panic')
        };
    }
    "#);
}

#[test]
fn test_manual_if_with_additional_instructions_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r#"
    "#);
}

#[test]
fn test_manual_if_with_additional_instructions_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r#"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        let _a = if let Option::Some(val) = opt_val {
            let val = val + 1;
            val
        } else {
            core::panic_with_felt252('panic')
        };
    }
    "#);
}

#[test]
fn test_manual_result_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_RESULT_IF, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = if let Result::Ok(x) = res_val {
      |  ______________-
    5 | |         x
    6 | |     } else {
    7 | |         core::panic_with_felt252('err')
    8 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_result_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_RESULT_IF, @r#"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let _a = res_val.expect('err');
    }
    "#);
}

#[test]
fn test_manual_match_result_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_MATCH_RESULT, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = match res_val {
      |  ______________-
    5 | |         Result::Ok(val) => val,
    6 | |         Result::Err(_) => core::panic_with_felt252('error')
    7 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_match_result_fixer() {
    test_lint_fixer!(TEST_MANUAL_MATCH_RESULT, @r#"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let _a = res_val.expect('error');
    }
    "#);
}

#[test]
fn test_manual_match_result_with_unwrapped_error_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_MATCH_RESULT_WITH_UNWRAPPED_ERROR, @r"
    warning: Plugin diagnostic: Manual match for expect detected. Consider using `expect()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = match res_val {
      |  ______________-
    5 | |         Result::Ok(val) => val,
    6 | |         Result::Err(err) => core::panic_with_felt252(err)
    7 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_manual_match_result_with_unwrapped_error_fixer() {
    test_lint_fixer!(TEST_MANUAL_MATCH_RESULT_WITH_UNWRAPPED_ERROR, @r#"
    fn main() {
        let res_val: Result<i32> = Result::Err('err');
        let _a = res_val.expect(err);
    }
    "#);
}
