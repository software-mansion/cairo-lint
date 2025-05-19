use crate::{test_lint_diagnostics, test_lint_fixer};

const WHILE_LOOP_EXIT_CONDITION_LESS_THAN: &str = r#"
fn main() {
    let mut a = 1_u32;
    while a < 10 {
        a += 1;
    }
}
"#;

const WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL: &str = r#"
fn main() {
    let mut a = 1_u32;
    while a <= 10 {
        a += 1;
    }
}
"#;

const WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL_ALLOWED: &str = r#"
fn main() {
    let mut a = 1_u32;
    #[allow(inefficient_while_comp)]
    while a <= 10 {
        a += 1;
    }
}
"#;

const WHILE_LOOP_EXIT_CONDITION_GREATER_THAN: &str = r#"
fn main() {
    let mut a = 10_u32;
    while a > 0 {
        a -= 1;
    }
}
"#;

const WHILE_LOOP_EXIT_CONDITION_GREATER_THAN_OR_EQUAL: &str = r#"
fn main() {
    let mut a = 10_i32;
    while a >= 0 {
        a -= 1;
    }
}
"#;

const WHILE_LOOP_EXIT_CONDITION_NESTED: &str = r#"
fn main() {
    let mut a = 0_u32;
    let mut b = 0_u32;
    while a < 10 && b < 5 {
        a += 1;
        if a % 2 == 0 {
            b += 1
        }
    }
}
"#;

#[test]
fn while_loop_exit_condition_less_than_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN, @r"
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:4:11
        while a < 10 {
              ^^^^^^
    ");
}

#[test]
fn while_loop_exit_condition_less_than_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN, @r#"
    fn main() {
        let mut a = 1_u32;
        while a < 10 {
            a += 1;
        }
    }
    "#);
}

#[test]
fn while_loop_exit_condition_less_than_or_equal_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL, @r"
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:4:11
        while a <= 10 {
              ^^^^^^^
    ");
}

#[test]
fn while_loop_exit_condition_less_than_or_equal_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL, @r#"
    fn main() {
        let mut a = 1_u32;
        while a <= 10 {
            a += 1;
        }
    }
    "#);
}

#[test]
fn while_loop_exit_condition_less_than_or_equal_allowed_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL_ALLOWED, @r#"
    "#);
}

#[test]
fn while_loop_exit_condition_less_than_or_equal_allowed_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_LESS_THAN_OR_EQUAL_ALLOWED, @r#"
    fn main() {
        let mut a = 1_u32;
        #[allow(inefficient_while_comp)]
        while a <= 10 {
            a += 1;
        }
    }
    "#);
}

#[test]
fn while_loop_exit_condition_greater_than_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_GREATER_THAN, @r"
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:4:11
        while a > 0 {
              ^^^^^
    ");
}

#[test]
fn while_loop_exit_condition_greater_than_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_GREATER_THAN, @r#"
    fn main() {
        let mut a = 10_u32;
        while a > 0 {
            a -= 1;
        }
    }
    "#);
}

#[test]
fn while_loop_exit_condition_greater_than_or_equal_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_GREATER_THAN_OR_EQUAL, @r"
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:4:11
        while a >= 0 {
              ^^^^^^
    ");
}

#[test]
fn while_loop_exit_condition_greater_than_or_equal_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_GREATER_THAN_OR_EQUAL, @r#"
    fn main() {
        let mut a = 10_i32;
        while a >= 0 {
            a -= 1;
        }
    }
    "#);
}

#[test]
fn while_loop_exit_condition_nested_diagnostics() {
    test_lint_diagnostics!(WHILE_LOOP_EXIT_CONDITION_NESTED, @r"
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:5:11
        while a < 10 && b < 5 {
              ^^^^^^
    Plugin diagnostic: using [`<`, `<=`, `>=`, `>`] exit conditions is inefficient. Consider switching to `!=` or using ArrayTrait::multi_pop_front.
     --> lib.cairo:5:21
        while a < 10 && b < 5 {
                        ^^^^^
    ");
}

#[test]
fn while_loop_exit_condition_nested_fixer() {
    test_lint_fixer!(WHILE_LOOP_EXIT_CONDITION_NESTED, @r"
    fn main() {
        let mut a = 0_u32;
        let mut b = 0_u32;
        while a < 10 && b < 5 {
            a += 1;
            if a % 2 == 0 {
                b += 1
            }
        }
    }
    ");
}
