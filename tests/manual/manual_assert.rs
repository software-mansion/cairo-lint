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

const TEST_BASIC_MANUAL_ASSERT_WITH_TAIL: &str = r#"
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5")
    }
}
"#;

const TEST_BASIC_MANUAL_ASSERT_WITH_TAIL_ALLOWED: &str = r#"
fn main() {
    let a = 5;
    #[allow(manual_assert)]
    if a == 5 {
        panic!("a shouldn't be equal to 5")
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
    test_lint_diagnostics!(TEST_BASIC_MANUAL_ASSERT_WITH_TAIL, @r#"
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
    test_lint_diagnostics!(TEST_BASIC_MANUAL_ASSERT_WITH_TAIL_ALLOWED, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:6:9
            panic!("a shouldn't be equal to 5")
            ^^^^^
    "#);
}
