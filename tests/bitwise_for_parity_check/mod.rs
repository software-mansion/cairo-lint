use crate::{test_lint_diagnostics, test_lint_fixer};

const WITH_SINGLE_VARIABLE: &str = r#"
fn main() {
    let _a = 200_u32 & 1;
}
"#;

const WITH_MULTIPLE_VARIABLES: &str = r#"
fn main() {
    let x = 150_u32;
    let y = 47;
    let _result = (x & 1) + (y & 1);
}
"#;

const WITH_MULTIPLE_VARIABLES_ALLOWED: &str = r#"
fn main() {
    let x = 150_u32;
    let y = 47;
    #[allow(bitwise_for_parity_check)]
    let _result = (x & 1) + (y & 1);
}
"#;

const IN_A_LOOP: &str = r#"
fn main() {
    for i in 0..10_u8 {
        let y = i & 1;
        println!("{}", y);
    }
}
"#;

const WITH_CONDITIONAL_LOGIC: &str = r#"
fn main() {
    let x = 17_u32;
    if (x & 1) == 1 {
        println!("Odd number");
    } else {
        println!("Even number");
    }
}
"#;

const WITH_CONDITIONAL_LOGIC_ALLOWED: &str = r#"
fn main() {
    let x = 17_u32;
    #[allow(bitwise_for_parity_check)]
    if (x & 1) == 1 {
        println!("Odd number");
    } else {
        println!("Even number");
    }
}
"#;

#[test]
fn with_single_variable_diagnostics() {
    test_lint_diagnostics!(WITH_SINGLE_VARIABLE, @r"
    warning: Plugin diagnostic: You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.
     --> lib.cairo:3:14
      |
    3 |     let _a = 200_u32 & 1;
      |              -----------
      |
    ");
}

#[test]
fn with_single_variable_fixer() {
    test_lint_fixer!(WITH_SINGLE_VARIABLE, @r#"
    fn main() {
        let _a = 200_u32 & 1;
    }
    "#);
}

#[test]
fn with_multiple_variables_diagnostics() {
    test_lint_diagnostics!(WITH_MULTIPLE_VARIABLES, @r"
    warning: Plugin diagnostic: You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.
     --> lib.cairo:5:20
      |
    5 |     let _result = (x & 1) + (y & 1);
      |                    -----
      |
    warning: Plugin diagnostic: You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.
     --> lib.cairo:5:30
      |
    5 |     let _result = (x & 1) + (y & 1);
      |                              -----
      |
    ");
}

#[test]
fn with_multiple_variables_fixer() {
    test_lint_fixer!(WITH_MULTIPLE_VARIABLES, @r#"
    fn main() {
        let x = 150_u32;
        let y = 47;
        let _result = (x & 1) + (y & 1);
    }
    "#);
}

#[test]
fn with_multiple_variables_allowed_diagnostics() {
    test_lint_diagnostics!(WITH_MULTIPLE_VARIABLES_ALLOWED, @r#"
    "#);
}

#[test]
fn with_multiple_variables_allowed_fixer() {
    test_lint_fixer!(WITH_MULTIPLE_VARIABLES_ALLOWED, @r#"
    fn main() {
        let x = 150_u32;
        let y = 47;
        #[allow(bitwise_for_parity_check)]
        let _result = (x & 1) + (y & 1);
    }
    "#);
}

#[test]
fn in_a_loop_diagnostics() {
    test_lint_diagnostics!(IN_A_LOOP, @r"
    warning: Plugin diagnostic: You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.
     --> lib.cairo:4:17
      |
    4 |         let y = i & 1;
      |                 -----
      |
    ");
}

#[test]
fn in_a_loop_fixer() {
    test_lint_fixer!(IN_A_LOOP, @r#"
    fn main() {
        for i in 0..10_u8 {
            let y = i & 1;
            println!("{}", y);
        }
    }
    "#);
}

#[test]
fn with_conditional_logic_diagnostics() {
    test_lint_diagnostics!(WITH_CONDITIONAL_LOGIC, @r"
    warning: Plugin diagnostic: You seem to be trying to use `&` for parity check. Consider using `DivRem::div_rem()` instead.
     --> lib.cairo:4:9
      |
    4 |     if (x & 1) == 1 {
      |         -----
      |
    ");
}

#[test]
fn with_conditional_logic_fixer() {
    test_lint_fixer!(WITH_CONDITIONAL_LOGIC, @r#"
    fn main() {
        let x = 17_u32;
        if (x & 1) == 1 {
            println!("Odd number");
        } else {
            println!("Even number");
        }
    }
    "#);
}

#[test]
fn with_conditional_logic_allowed_diagnostics() {
    test_lint_diagnostics!(WITH_CONDITIONAL_LOGIC_ALLOWED, @r#"
    "#);
}

#[test]
fn with_conditional_logic_allowed_fixer() {
    test_lint_fixer!(WITH_CONDITIONAL_LOGIC_ALLOWED, @r#"
    fn main() {
        let x = 17_u32;
        #[allow(bitwise_for_parity_check)]
        if (x & 1) == 1 {
            println!("Odd number");
        } else {
            println!("Even number");
        }
    }
    "#);
}
