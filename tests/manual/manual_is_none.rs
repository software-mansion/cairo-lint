use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_NONE: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    // This is just a variable.
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
    // This is just a variable.
    let _foo = match foo {
        Option::Some(_) => false,
        Option::None => true,
    };
}
"#;

const TEST_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
    let foo: Option::<i32> = Option::None;
    // This is just a variable.
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
  // This is just a variable.
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
    // This is just a variable.
    let _a = match foo(a) {
        Option::Some(_) => false,
        Option::None => true
    };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
    let opt_val: Option<i32> = Option::None;
    // This is just a variable.
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
    // This is just a variable.
    let _a = if let Option::Some(_) = opt_val {
        val += 1;
        false
    } else {
        true
    };
}
"#;

const TEST_BASIC_IS_NONE_BLOCK: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(_) => {
            false
        },
        Option::None => {
            true
        },
    };
}
"#;

#[test]
fn test_basic_is_none_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_NONE, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:5:16-8:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_is_none_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_NONE, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_none();
    }
    ");
}

#[test]
fn test_basic_is_none_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_NONE_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_is_none_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_NONE_ALLOWED, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        #[allow(manual_is_none)]
        // This is just a variable.
        let _foo = match foo {
            Option::Some(_) => false,
            Option::None => true,
        };
    }
    ");
}

#[test]
fn test_with_comment_in_some_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_SOME, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
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
        let _foo = foo.is_none();
    }
    ");
}

#[test]
fn test_with_comment_in_none_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_NONE, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:5:14-11:3
        let _foo = match foo {
     ______________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn test_with_comment_in_none_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_NONE, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_none();
    }
    ");
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:8:14-11:5
          let _a = match foo(a) {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_match_expression_is_a_function_fixer() {
    test_lint_fixer!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    fn foo(a: u256) -> Option<u256> {
        Option::Some(a)
    }
    fn main() {
        let a: u256 = 0;
        // This is just a variable.
        let _a = foo(a).is_none();
    }
    ");
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:5:14-9:5
          let _a = if let Option::Some(_) = opt_val {
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
    test_lint_fixer!(TEST_MANUAL_IF_WITH_ADDITIONAL_INSTRUCTIONS, @r"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        let mut val = 1;
        // This is just a variable.
        let _a = if let Option::Some(_) = opt_val {
            val += 1;
            false
        } else {
            true
        };
    }
    ");
}

#[test]
fn test_basic_is_none_block_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_NONE_BLOCK, @r"
    Plugin diagnostic: Manual match for `is_none` detected. Consider using `is_none()` instead
     --> lib.cairo:5:16-12:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_is_none_block_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_NONE_BLOCK, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_none();
    }
    ");
}
