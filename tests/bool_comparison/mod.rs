use crate::{test_lint_diagnostics, test_lint_fixer};

const COMPARISON_WITH_TRUE: &str = r#"
fn main() {
    let x = true;
    if x == true {
        println!("x is true");
    }
}
"#;

const COMPARISON_WITH_TRUE_ON_LHS: &str = r#"
fn main() {
    let x = true;
    if true == x {
        println!("x is true");
    }
}
"#;

const COMPARISON_WITH_FALSE: &str = r#"
fn main() {
    let x = true;
    if x == false {
        println!("x is false");
    }
}
"#;

const COMPARISON_WITH_FALSE_ALLOWED: &str = r#"
fn main() {
    let x = true;
    #[allow(bool_comparison)]
    if x == false {
        println!("x is false");
    }
}
"#;

const COMPARISON_WITH_FALSE_ON_LHS: &str = r#"
fn main() {
    let x = true;
    if false == x {
        println!("x is false");
    }
}
"#;

const NEGATED_COMPARISON_WITH_TRUE: &str = r#"
fn main() {
    let x = true;
    if x != true {
        println!("x is not true");
    }
}
"#;

const NEGATED_COMPARISON_WITH_TRUE_ON_LHS: &str = r#"
fn main() {
    let x = true;
    if true != x {
        println!("x is not true");
    }
}
"#;

const NEGATED_COMPARISON_WITH_FALSE: &str = r#"
fn main() {
    let x = true;
    if x != false {
        println!("x is not false");
    }
}
"#;

const NEGATED_COMPARISON_WITH_FALSE_ON_LHS: &str = r#"
fn main() {
    let x = true;
    if false != x {
        println!("x is not false");
    }
}
"#;

#[test]
fn comparison_with_true_diagnostics() {
    test_lint_diagnostics!(COMPARISON_WITH_TRUE, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if x == true {
      |        ---------
      |
    ");
}

#[test]
fn comparison_with_true_fixer() {
    test_lint_fixer!(COMPARISON_WITH_TRUE, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        }
    }
    "#);
}

#[test]
fn comparison_with_true_on_lhs_diagnostics() {
    test_lint_diagnostics!(COMPARISON_WITH_TRUE_ON_LHS, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if true == x {
      |        ---------
      |
    ");
}

#[test]
fn comparison_with_true_on_lhs_fixer() {
    test_lint_fixer!(COMPARISON_WITH_TRUE_ON_LHS, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        }
    }
    "#);
}

#[test]
fn comparison_with_false_diagnostics() {
    test_lint_diagnostics!(COMPARISON_WITH_FALSE, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if x == false {
      |        ----------
      |
    ");
}

#[test]
fn comparison_with_false_fixer() {
    test_lint_fixer!(COMPARISON_WITH_FALSE, @r#"
    fn main() {
        let x = true;
        if !x {
            println!("x is false");
        }
    }
    "#);
}

#[test]
fn comparison_with_false_allowed_diagnostics() {
    test_lint_diagnostics!(COMPARISON_WITH_FALSE_ALLOWED, @r#"
    "#);
}

#[test]
fn comparison_with_false_allowed_fixer() {
    test_lint_fixer!(COMPARISON_WITH_FALSE_ALLOWED, @r#"
    fn main() {
        let x = true;
        #[allow(bool_comparison)]
        if x == false {
            println!("x is false");
        }
    }
    "#);
}

#[test]
fn comparison_with_false_on_lhs_diagnostics() {
    test_lint_diagnostics!(COMPARISON_WITH_FALSE_ON_LHS, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if false == x {
      |        ----------
      |
    ");
}

#[test]
fn comparison_with_false_on_lhs_fixer() {
    test_lint_fixer!(COMPARISON_WITH_FALSE_ON_LHS, @r#"
    fn main() {
        let x = true;
        if !x {
            println!("x is false");
        }
    }
    "#);
}

#[test]
fn negated_comparison_with_true_diagnostics() {
    test_lint_diagnostics!(NEGATED_COMPARISON_WITH_TRUE, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if x != true {
      |        ---------
      |
    ");
}

#[test]
fn negated_comparison_with_true_fixer() {
    test_lint_fixer!(NEGATED_COMPARISON_WITH_TRUE, @r#"
    fn main() {
        let x = true;
        if !x {
            println!("x is not true");
        }
    }
    "#);
}

#[test]
fn negated_comparison_with_true_on_lhs_diagnostics() {
    test_lint_diagnostics!(NEGATED_COMPARISON_WITH_TRUE_ON_LHS, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if true != x {
      |        ---------
      |
    ");
}

#[test]
fn negated_comparison_with_true_on_lhs_fixer() {
    test_lint_fixer!(NEGATED_COMPARISON_WITH_TRUE_ON_LHS, @r#"
    fn main() {
        let x = true;
        if !x {
            println!("x is not true");
        }
    }
    "#);
}

#[test]
fn negated_comparison_with_false_diagnostics() {
    test_lint_diagnostics!(NEGATED_COMPARISON_WITH_FALSE, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if x != false {
      |        ----------
      |
    ");
}

#[test]
fn negated_comparison_with_false_fixer() {
    test_lint_fixer!(NEGATED_COMPARISON_WITH_FALSE, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is not false");
        }
    }
    "#);
}

#[test]
fn negated_comparison_with_false_on_lhs_diagnostics() {
    test_lint_diagnostics!(NEGATED_COMPARISON_WITH_FALSE_ON_LHS, @r"
    warning: Plugin diagnostic: Unnecessary comparison with a boolean value. Use the variable directly.
     --> lib.cairo:4:8
      |
    4 |     if false != x {
      |        ----------
      |
    ");
}

#[test]
fn negated_comparison_with_false_on_lhs_fixer() {
    test_lint_fixer!(NEGATED_COMPARISON_WITH_FALSE_ON_LHS, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is not false");
        }
    }
    "#);
}
