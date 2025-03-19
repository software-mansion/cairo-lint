use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_LOOP_WITH_BREAK: &str = r#"
fn main() {
    let mut x: u16 = 0;
    loop {
        if x == 10 {
            break;
        }
        x += 1;
    }
}
"#;

const SIMPLE_LOOP_WITH_BREAK_WITH_COMMENT: &str = r#"
fn main() {
    let mut x: u16 = 0;
    loop {
        if x == 10 {
            break;
        }
        x += 1;
    }
}
"#;

const LOOP_WITH_COMPARISON_CONDITION: &str = r#"
fn main() {
    let mut counter: u16 = 0;
    loop {
        if counter > 5 {
            break;
        }
        counter += 1;
    }
}
"#;

const LOOP_WITH_NEGATIVE_CONDITION: &str = r#"
fn main() {
    let mut value: u16 = 100;
    loop {
        if value < 0 {
            break;
        }
        value -= 10;
    }
}
"#;

const LOOP_WITH_ARITHMETIC_CONDITION: &str = r#"
fn main() {
    let mut x: u16 = 5;
    loop {
        if x * 2 >= 20 {
            break;
        }
        x += 1;
    }
}
"#;

const LOOP_WITH_ARITHMETIC_CONDITION_ALLOWED: &str = r#"
fn main() {
    let mut x: u16 = 5;
    #[allow(loop_for_while)]
    // This is a loop.
    loop {
        if x * 2 >= 20 {
            break;
        }
        x += 1;
    }
}
"#;

const LOOP_WITH_MULTIPLE_CONDITIONS: &str = r#"
fn main() {
    let mut a: u16 = 0;
    let mut b: u16 = 0;
    loop {
        if a > 10 && b < 5 {
            break;
        }
        a += 1;
        b += 1;
    }
}
"#;

const LOOP_WITH_ARITHMETIC_CONDITION_AND_ELSE_BLOCK: &str = r#"
fn main() {
    let mut x: u16 = 5;
    loop {
        if x * 2 >= 20 {
            break;
        } else {
            x += 1;
        }
    }
}
"#;

const LOOP_WITH_MULTIPLE_CONDITION_INSIDE_IF_BLOCK: &str = r#"
fn main() {
    let mut x: u16 = 5;
    loop {
        if x * 2 >= 20 {
            if x > 30 {
                break;
            }
        } else {
            x += 1;
        }
    }
}
"#;

const LOOP_WITH_ARITHMETIC_CONDITION_AND_SECOND_INCREMENT: &str = r#"
fn main() {
    let mut x: u16 = 5;
    // This is a loop.
    loop {
        if x * 2 >= 20 {
            // This is a break statement.
            break;
        } else {
            // This just increments the x variable.
            x += 1;
        }
        // Same as above.
        x += 1;
    }
}
"#;

const LOOP_WITH_MULTIPLE_INCREMENTS_AND_COMPARISON: &str = r#"
fn main() {
    let mut x: u16 = 0;
    let mut y: u16 = 10;
    loop {
        if x > 5 || y == 0 {
            break;
        }
        x += 2;
        y -= 1;
    }
}
"#;

const LOOP_WITH_CONDITION_DEPENDING_ON_EXTERNAL_VARIABLE: &str = r#"
fn main() {
    let mut x: u16 = 0;
    let limit: u16 = 15;
    loop {
        if x >= limit {
            break;
        }
        x += 2;
    }
}
"#;

const ADVANCED_LOOP_WITH_BREAK_IN_THE_MIDDLE: &str = r#"
fn main() -> u32 {
    let mut exponent: u32 = 3;
    let two: u32 = 2;
    let mut result: u32 = 0;
    let mut base: u32 = 0;
    loop {
        if exponent % two != 0 {
            let new_result = 10;
            result = new_result;
        }

        exponent = exponent / two;

        if exponent == 0 {
            break result;
        }

        let new_base = 2;

        base = new_base;
    };
    1
}
"#;

const SIMPLE_LOOP_WITH_BREAK_AT_THE_END: &str = r#"
fn main() {
    let mut x: u16 = 0;
    loop {
        x += 1;
        if x == 10 {
            break;
        }
    }
}
"#;

const SIMPLE_LOOP_WITH_BREAK_WITH_RETURN_VALUE: &str = r#"
fn main() -> u16 {
    let mut x: u16 = 0;
    loop {
        if x == 10 {
            break x;
        }
        x += 1;
    }
}
"#;

#[test]
fn simple_loop_with_break_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_WITH_BREAK, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-9:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn simple_loop_with_break_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_WITH_BREAK, @r#"
    fn main() {
        let mut x: u16 = 0;
        while x != 10 {
            x += 1;
        }
    }
    "#);
}

#[test]
fn simple_loop_with_break_with_comment_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_WITH_BREAK_WITH_COMMENT, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-9:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn simple_loop_with_break_with_comment_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_WITH_BREAK_WITH_COMMENT, @r#"
    fn main() {
        let mut x: u16 = 0;
        while x != 10 {
            x += 1;
        }
    }
    "#);
}

#[test]
fn loop_with_comparison_condition_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_COMPARISON_CONDITION, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-9:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_comparison_condition_fixer() {
    test_lint_fixer!(LOOP_WITH_COMPARISON_CONDITION, @r#"
    fn main() {
        let mut counter: u16 = 0;
        while counter <= 5 {
            counter += 1;
        }
    }
    "#);
}

#[test]
fn loop_with_negative_condition_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_NEGATIVE_CONDITION, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-9:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_negative_condition_fixer() {
    test_lint_fixer!(LOOP_WITH_NEGATIVE_CONDITION, @r#"
    fn main() {
        let mut value: u16 = 100;
        while value >= 0 {
            value -= 10;
        }
    }
    "#);
}

#[test]
fn loop_with_arithmetic_condition_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_ARITHMETIC_CONDITION, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-9:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_arithmetic_condition_fixer() {
    test_lint_fixer!(LOOP_WITH_ARITHMETIC_CONDITION, @r#"
    fn main() {
        let mut x: u16 = 5;
        while x * 2 < 20 {
            x += 1;
        }
    }
    "#);
}

#[test]
fn loop_with_arithmetic_condition_allowed_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_ARITHMETIC_CONDITION_ALLOWED, @r#"
    "#);
}

#[test]
fn loop_with_arithmetic_condition_allowed_fixer() {
    test_lint_fixer!(LOOP_WITH_ARITHMETIC_CONDITION_ALLOWED, @r"
    fn main() {
        let mut x: u16 = 5;
        #[allow(loop_for_while)]
        // This is a loop.
        loop {
            if x * 2 >= 20 {
                break;
            }
            x += 1;
        }
    }
    ");
}

#[test]
fn loop_with_multiple_conditions_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_MULTIPLE_CONDITIONS, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:5:5-11:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_multiple_conditions_fixer() {
    test_lint_fixer!(LOOP_WITH_MULTIPLE_CONDITIONS, @r#"
    fn main() {
        let mut a: u16 = 0;
        let mut b: u16 = 0;
        while a <= 10 || b >= 5 {
            a += 1;
            b += 1;
        }
    }
    "#);
}

#[test]
fn loop_with_arithmetic_condition_and_else_block_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_ARITHMETIC_CONDITION_AND_ELSE_BLOCK, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:4:5-10:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_arithmetic_condition_and_else_block_fixer() {
    test_lint_fixer!(LOOP_WITH_ARITHMETIC_CONDITION_AND_ELSE_BLOCK, @r#"
    fn main() {
        let mut x: u16 = 5;
        while x * 2 < 20 {
            x += 1;
        }
    }
    "#);
}

#[test]
fn loop_with_multiple_condition_inside_if_block_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_MULTIPLE_CONDITION_INSIDE_IF_BLOCK, @r#"
    "#);
}

#[test]
fn loop_with_multiple_condition_inside_if_block_fixer() {
    test_lint_fixer!(LOOP_WITH_MULTIPLE_CONDITION_INSIDE_IF_BLOCK, @r#"
    fn main() {
        let mut x: u16 = 5;
        loop {
            if x * 2 >= 20 {
                if x > 30 {
                    break;
                }
            } else {
                x += 1;
            }
        }
    }
    "#);
}

#[test]
fn loop_with_arithmetic_condition_and_second_increment_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_ARITHMETIC_CONDITION_AND_SECOND_INCREMENT, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:5:5-15:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_arithmetic_condition_and_second_increment_fixer() {
    test_lint_fixer!(LOOP_WITH_ARITHMETIC_CONDITION_AND_SECOND_INCREMENT, @r"
    fn main() {
        let mut x: u16 = 5;
        // This is a loop.
        while x * 2 < 20 {
            // This just increments the x variable.
            x += 1;
            // Same as above.
            x += 1;
        }
    }
    ");
}

#[test]
fn loop_with_multiple_increments_and_comparison_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_MULTIPLE_INCREMENTS_AND_COMPARISON, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:5:5-11:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_multiple_increments_and_comparison_fixer() {
    test_lint_fixer!(LOOP_WITH_MULTIPLE_INCREMENTS_AND_COMPARISON, @r#"
    fn main() {
        let mut x: u16 = 0;
        let mut y: u16 = 10;
        while x <= 5 && y != 0 {
            x += 2;
            y -= 1;
        }
    }
    "#);
}

#[test]
fn loop_with_condition_depending_on_external_variable_diagnostics() {
    test_lint_diagnostics!(LOOP_WITH_CONDITION_DEPENDING_ON_EXTERNAL_VARIABLE, @r"
    Plugin diagnostic: you seem to be trying to use `loop`. Consider replacing this `loop` with a `while` loop for clarity and conciseness
     --> lib.cairo:5:5-10:5
          loop {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn loop_with_condition_depending_on_external_variable_fixer() {
    test_lint_fixer!(LOOP_WITH_CONDITION_DEPENDING_ON_EXTERNAL_VARIABLE, @r#"
    fn main() {
        let mut x: u16 = 0;
        let limit: u16 = 15;
        while x < limit {
            x += 2;
        }
    }
    "#);
}

#[test]
fn advanced_loop_with_break_in_the_middle_diagnostics() {
    test_lint_diagnostics!(ADVANCED_LOOP_WITH_BREAK_IN_THE_MIDDLE, @"");
}

#[test]
fn advanced_loop_with_break_in_the_middle_fixer() {
    test_lint_fixer!(ADVANCED_LOOP_WITH_BREAK_IN_THE_MIDDLE, @r"
    fn main() -> u32 {
        let mut exponent: u32 = 3;
        let two: u32 = 2;
        let mut result: u32 = 0;
        let mut base: u32 = 0;
        loop {
            if exponent % two != 0 {
                let new_result = 10;
                result = new_result;
            }

            exponent = exponent / two;

            if exponent == 0 {
                break result;
            }

            let new_base = 2;

            base = new_base;
        };
        1
    }
    ");
}

#[test]
fn simple_loop_with_break_at_the_end_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_WITH_BREAK_AT_THE_END, @r"");
}

#[test]
fn simple_loop_with_break_at_the_end_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_WITH_BREAK_AT_THE_END, @r"
    fn main() {
        let mut x: u16 = 0;
        loop {
            x += 1;
            if x == 10 {
                break;
            }
        }
    }
    ");
}

#[test]
fn simple_loop_with_break_with_return_value_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_WITH_BREAK_WITH_RETURN_VALUE, @"");
}

#[test]
fn simple_loop_with_break_with_return_value_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_WITH_BREAK_WITH_RETURN_VALUE, @r"
    fn main() -> u16 {
        let mut x: u16 = 0;
        loop {
            if x == 10 {
                break x;
            }
            x += 1;
        }
    }
    ");
}
