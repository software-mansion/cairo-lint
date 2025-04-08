use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_BASIC_MANUAL_ASSERT: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5");
    }
}
"#;

const TEST_BASIC_MANUAL_ASSERT_ALLOWED: &str = r#"
fn main() {
    let a = 5;
    #[allow(manual_assert)]
    if a == 5 {
        panic!("a shouldn't be equal to 5");
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_TAIL: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5")
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_TAIL_ALLOWED: &str = r#"
fn main() {
    let a = 5;
    #[allow(manual_assert)]
    if a == 5 {
        panic!("a shouldn't be equal to 5")
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_OTHER_EXPRS: &str = r#"
fn main() -> felt252 {
    let a = 5;
    if a == 5 {
        return a;
        panic!("a shouldn't be equal to 5");
    }
    a
}
"#;

const TEST_MANUAL_ASSERT_WITH_OTHER_EXPRS_AND_TAIL: &str = r#"
fn main() {
    let mut a = 5;
    if a == 5 {
        a = 6;
        panic!("a shouldn't be equal to 5")
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to {}", a);
    }
}
"#;
const TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_ALLOWED: &str = r#"
fn main() {
    let a = 5;
    #[allow(manual_assert)]
    if a == 5 {
        panic!("a shouldn't be equal to {}", a);
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to {}", a)
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL_ALLOWED: &str = r#"
fn main() {
    let a = 5;
    #[allow(manual_assert)]
    if a == 5 {
        panic!("a shouldn't be equal to {}", a)
    }
}
"#;

#[test]
fn test_basic_manual_assert_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MANUAL_ASSERT, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:9
            panic!("a shouldn't be equal to 5");
            ^^^^^
    Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
     --> lib.cairo:4:5-6:5
          if a == 5 {
     _____^
    |         panic!("a shouldn't be equal to 5");
    |     }
    |_____^
    "#);
}

#[test]
fn test_basic_manual_assert_fixer() {
    test_lint_fixer!(TEST_BASIC_MANUAL_ASSERT, @r#"
    fn main() {
        let a = 5;
        if a == 5 {
            panic!("a shouldn't be equal to 5");
        }
    }
    "#);
}

#[test]
fn test_basic_manual_assert_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_BASIC_MANUAL_ASSERT_ALLOWED, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5");
            ^^^^^
    "#);
}

#[test]
fn test_basic_manual_assert_with_tail_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_TAIL, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:9
            panic!("a shouldn't be equal to 5")
            ^^^^^
    Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
     --> lib.cairo:4:5-6:5
          if a == 5 {
     _____^
    |         panic!("a shouldn't be equal to 5")
    |     }
    |_____^
    "#);
}

#[test]
fn test_basic_manual_assert_with_tail_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_TAIL_ALLOWED, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5")
            ^^^^^
    "#);
}

#[test]
fn test_basic_manual_assert_with_other_exprs_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_OTHER_EXPRS, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5");
            ^^^^^
    "#);
}

#[test]
fn test_basic_manual_assert_with_other_exprs_and_tail_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_OTHER_EXPRS_AND_TAIL, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5")
            ^^^^^
    "#);
}

#[test]
fn test_manual_assert_with_multiple_panic_args_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS, @r#"
    Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
     --> lib.cairo:4:5-6:5
          if a == 5 {
     _____^
    |         panic!("a shouldn't be equal to {}", a);
    |     }
    |_____^
    "#);
}

#[test]
fn test_manual_assert_with_multiple_panic_args_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_ALLOWED, @r#""#);
}

#[test]
fn test_manual_assert_with_multiple_panic_args_and_tail_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL, @r#"
  Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
   --> lib.cairo:4:5-6:5
        if a == 5 {
   _____^
  |         panic!("a shouldn't be equal to {}", a)
  |     }
  |_____^
  "#);
}

#[test]
fn test_manual_assert_with_multiple_panic_args_and_tail_allowed_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL_ALLOWED, @r#""#);
}
