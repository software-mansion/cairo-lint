use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_NONE: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(_) => false,
        Option::None => true,
    };
}
"#;

const TEST_BASIC_IS_NONE_ALLOWED: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    #[allow(manual_is_none)]
    let _foo = match foo {
        Option::Some(_) => false,
        Option::None => true,
    };
}
"#;

const TEST_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(_) => {
            // do something
            false
        },
        Option::None => true,
    };
}
"#;

const TEST_WITH_COMMENT_IN_NONE: &str = r#"
fn main() {
  let foo: Option::<i32> = Option::None;
  let _foo = match foo {
      Option::Some(_) => false,
      Option::None => {
          // do something
          true
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
        Option::Some(_) => false,
        Option::None => true
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    let _a = if let Option::Some(_) = opt_val {
        false
    } else {
        true
    };
}
"#;

const TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS: &str = r#"
fn main() {
    let opt_val: Option::<i32> = Option::None;
    let mut val = 1;
    let _a = if let Option::Some(_) = opt_val {
        val += 1;
        false
    } else {
        true
    };
}
"#;

#[test]
fn test_basic_is_none_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_NONE, @r"
    warning: Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:4:16
      |
    4 |       let _foo = match foo {
      |  ________________-
    5 | |         Option::Some(_) => false,
    6 | |         Option::None => true,
    7 | |     };
      | |_____-
      |
    ");
}

#[test]
fn test_basic_is_none_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_NONE, @r#"
    fn main() {
        let foo: Option::<i32> = Option::None;
        let _foo = foo.is_none();
    }
    "#);
}

#[test]
fn test_basic_is_none_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_NONE_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_is_none_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_NONE_ALLOWED, @r#"
    fn main() {
        let foo: Option::<i32> = Option::None;
        #[allow(manual_is_none)]
        let _foo = match foo {
            Option::Some(_) => false,
            Option::None => true,
        };
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
            Option::Some(_) => {
                // do something
                false
            },
            Option::None => true,
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
          Option::Some(_) => false,
          Option::None => {
              // do something
              true
          },
      };
    }
    "#);
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
      --> lib.cairo:7:14
       |
     7 |       let _a = match foo(a) {
       |  ______________-
     8 | |         Option::Some(_) => false,
     9 | |         Option::None => true
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
        let _a = foo(a).is_none();
    }
    "#);
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    warning: Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:4:14
      |
    4 |       let _a = if let Option::Some(_) = opt_val {
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
    test_lint_fixer!(TEST_MANUAL_IF, @r"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        let _a = opt_val.is_none();
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
    test_lint_fixer!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r#"
    fn main() {
        let opt_val: Option::<i32> = Option::None;
        let mut val = 1;
        let _a = if let Option::Some(_) = opt_val {
            val += 1;
            false
        } else {
            true
        };
    }
    "#);
}
