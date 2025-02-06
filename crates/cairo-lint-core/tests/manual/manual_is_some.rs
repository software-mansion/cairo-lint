use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_IS_SOME: &str = r#"
fn main() {
  let foo: Option::<i32> = Option::None;
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
  let _foo = match foo {
      Option::Some(_) => true,
      Option::None => false,
  };
}
"#;

const TEST_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
  let foo: Option::<i32> = Option::None;
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
  let _a = match foo(a) {
      Option::Some(_) => true,
      Option::None => false
  };
}
"#;

const TEST_MANUAL_IF: &str = r#"
fn main() {
  let opt_val: Option<i32> = Option::None;
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
  let _a = if let Option::Some(_) = opt_val {
      val += 1;
      false
  } else {
      true
  };
}
"#;

#[test]
fn test_basic_is_some_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_SOME, @r"
    warning: Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:4:14
      |
    4 |     let _foo = match foo {
      |  ______________-
    5 | |       Option::Some(_) => true,
    6 | |       Option::None => false,
    7 | |   };
      | |___-
      |
    ");
}

#[test]
fn test_basic_is_some_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_SOME, @r#"
    fn main() {
      let foo: Option::<i32> = Option::None;
      let _foo = foo.is_some();
    }
    "#);
}

#[test]
fn test_basic_is_some_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_IS_SOME_ALLOWED, @r#"
    "#);
}

#[test]
fn test_basic_is_some_allowed_fixer() {
    test_lint_fixer!(TEST_BASIC_IS_SOME_ALLOWED, @r#"
    #[allow(manual_is_some)]
    fn main() {
      let foo: Option::<i32> = Option::None;
      let _foo = match foo {
          Option::Some(_) => true,
          Option::None => false,
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
              true
          },
          Option::None => false,
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
          Option::Some(_) => true,
          Option::None => {
              // do something
              false
          },
      };
    }
    "#);
}

#[test]
fn test_match_expression_is_a_function_diagnostics() {
    test_lint_diagnostics!(TEST_MATCH_EXPRESSION_IS_A_FUNCTION, @r"
    warning: Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
      --> lib.cairo:7:12
       |
     7 |     let _a = match foo(a) {
       |  ____________-
     8 | |       Option::Some(_) => true,
     9 | |       Option::None => false
    10 | |   };
       | |___-
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
      let _a = foo(a).is_some();
    }
    "#);
}

#[test]
fn test_manual_if_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IF, @r"
    warning: Plugin diagnostic: Manual match for `is_some` detected. Consider using `is_some()` instead
     --> lib.cairo:4:12
      |
    4 |     let _a = if let Option::Some(_) = opt_val {
      |  ____________-
    5 | |       true
    6 | |   } else {
    7 | |       false
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn test_manual_if_fixer() {
    test_lint_fixer!(TEST_MANUAL_IF, @r"
    fn main() {
      let opt_val: Option<i32> = Option::None;
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
