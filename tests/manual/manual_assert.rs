use crate::test_lint_diagnostics;

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

const TEST_MANUAL_ASSERT_WITH_MORE_THAN_ONE_STATEMENTS: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5");
        println!("a is {}", a);
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_MORE_THAN_ONE_STATEMENTS_BEFORE_PANIC: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        println!("a is {}", a);
        panic!("a shouldn't be equal to 5");
    }
}
"#;

const TEST_MANUAL_ASSERT_WITH_ELSE_BLOCK: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5");
    } else {
        println!("a is {}", a);
    }
}
"#;

const TEST_MANUAL_ASSERT_WITHIN_ELSE_BLOCK: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        println!("a is {}", a);
    } else {
        panic!("a should be equal to 5");
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
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:9
            panic!("a shouldn't be equal to {}", a);
            ^^^^^
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
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_ALLOWED, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to {}", a);
            ^^^^^
    "#);
}

#[test]
fn test_manual_assert_with_multiple_panic_args_and_tail_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:9
            panic!("a shouldn't be equal to {}", a)
            ^^^^^
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
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MULTIPLE_PANIC_ARGS_AND_TAIL_ALLOWED, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to {}", a)
            ^^^^^
    "#);
}

#[test]
fn test_manual_assert_with_more_than_one_statements() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MORE_THAN_ONE_STATEMENTS, @r#"
  Plugin diagnostic: Leaving `panic` in the code is discouraged.
   --> lib.cairo:5:9
          panic!("a shouldn't be equal to 5");
          ^^^^^
  Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
   --> lib.cairo:4:5-7:5
        if a == 5 {
   _____^
  | ...
  |     }
  |_____^
  "#);
}

#[test]
fn test_manual_assert_with_more_than_one_statements_before_panic() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_MORE_THAN_ONE_STATEMENTS_BEFORE_PANIC, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5");
            ^^^^^
    "#);
}

#[test]
fn test_manual_assert_with_else_block() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITH_ELSE_BLOCK, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:9
            panic!("a shouldn't be equal to 5");
            ^^^^^
    Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
     --> lib.cairo:4:5-8:5
          if a == 5 {
     _____^
    | ...
    |     }
    |_____^
    "#);
}

#[test]
fn test_manual_assert_within_else_block() {
    test_lint_diagnostics!(TEST_MANUAL_ASSERT_WITHIN_ELSE_BLOCK, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:7:9
            panic!("a should be equal to 5");
            ^^^^^
    Plugin diagnostic: Manual assert detected. Consider using assert!() macro instead.
     --> lib.cairo:4:5-8:5
          if a == 5 {
     _____^
    | ...
    |     }
    |_____^
    "#);
}
