use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_SOME: &str = r#"
fn main() {
  let foo: Option::<i32> = Option::None;
  // This is just a variable.
  let _foo = match foo {
      Option::Some(_) => true,
      Option::None => false,
  };
}
"#;

const TEST_BASIC_IS_SOME_ALLOWED: &str = r#"
#[allow(manual_is_some)]
fn main() {
  let foo: Option::<i32> = Option::None;
  // This is just a variable.
  let _foo = match foo {
      Option::Some(_) => true,
      Option::None => false,
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
          true
      },
      Option::None => false,
  };
}
"#;

const TEST_WITH_COMMENT_IN_NONE: &str = r#"
fn main() {
  let foo: Option::<i32> = Option::None;
  // This is just a variable.
  let _foo = match foo {
      Option::Some(_) => true,
      Option::None => {
          // do something
          false
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
      Option::Some(_) => true,
      Option::None => false
  };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
  let opt_val: Option<i32> = Option::None;
  // This is just a variable.
  let _a = if let Option::Some(_) = opt_val {
      true
  } else {
      false
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

const TEST_BASIC_IS_SOME_BLOCK: &str = r#"
fn main() {
    let foo: Option<i32> = Option::None;
    // This is just a variable.
    let _foo = match foo {
        Option::Some(_) => {
            true
        },
        Option::None => {
            false
        },
    };
}
"#;

const MATCH_WITH_REVERSED_ARMS: &str = r#"
fn main() {
    let a: Option<usize> = Option::None;
    let _ = match a {
        Option::None => false,
        Option::Some(_) => true,
    };
}
"#;

#[test]
fn test_basic_is_some_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_SOME, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:5:14-8:3
        let _foo = match foo {
     ______________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn test_basic_is_some_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_SOME, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_some();
    }
    ");
}

#[test]
fn test_basic_is_some_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_SOME_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_is_some_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_SOME_ALLOWED, @r"
    #[allow(manual_is_some)]
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = match foo {
            Option::Some(_) => true,
            Option::None => false,
        };
    }
    ");
}

#[test]
fn test_with_comment_in_some_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_SOME, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:5:14-11:3
        let _foo = match foo {
     ______________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn test_with_comment_in_some_fixer() {
    test_lint_fixer!(TEST_WITH_COMMENT_IN_SOME, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_some();
    }
    ");
}

#[test]
fn test_with_comment_in_none_diagnostics() {
    test_lint_diagnostics!(TEST_WITH_COMMENT_IN_NONE, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
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
        let _foo = foo.is_some();
    }
    ");
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:8:12-11:3
        let _a = match foo(a) {
     ____________^
    | ...
    |   };
    |___^
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
        let _a = foo(a).is_some();
    }
    ");
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:5:12-9:3
        let _a = if let Option::Some(_) = opt_val {
     ____________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r"
    fn main() {
        let opt_val: Option<i32> = Option::None;
        // This is just a variable.
        let _a = opt_val.is_some();
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
fn test_basic_is_some_block_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_SOME_BLOCK, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:5:16-12:5
          let _foo = match foo {
     ________________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn test_basic_is_some_block_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_SOME_BLOCK, @r"
    fn main() {
        let foo: Option<i32> = Option::None;
        // This is just a variable.
        let _foo = foo.is_some();
    }
    ");
}

#[test]
fn match_with_reversed_arms_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_REVERSED_ARMS, @r"
    Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
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
        let _ = a.is_some();
    }
    ");
}
