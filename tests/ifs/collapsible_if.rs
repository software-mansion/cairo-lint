use crate::{test_lint_diagnostics, test_lint_fixer};

const COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            println!("Hello");
        }
    }
}
"#;

const COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_WITH_COMMENT: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            // Just a comment.
            println!("Hello");
        }
    }
}
"#;

const COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_ALLOWED: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    #[allow(collapsible_if)]
    if x || z {
        if y && z {
            println!("Hello");
        }
    }
}
"#;

const COLLAPSIBLE_IF_WITH_COMBINABLE_CONDITIONS: &str = r#"
fn main() {
    let x = true;
    let z = true;

    if x {
        if z {
            println!("No fix here");
        }
    }
}
"#;

const COLLAPSIBLE_IF_IN_CONDITIONS_WITH_COMPLEX_EXPRESSIONS: &str = r#"
fn main() {
    let x = 3_u32;
    let y = 4_u32;
    let z = 5_u32;
    let a = 5_u32;
    let b = 2_u32;
    let c = 10_u32;

    if x + y > a {
        if z * b < c {
            println!("Complex conditions");
        }
    }
}
"#;

const COLLAPSIBLE_IF_WITH_FUNCTION_CALLS: &str = r#"
fn is_valid(_a: bool) -> bool { true } 
fn is_ready(_b: bool) -> bool { true } 

fn main() {
    if is_valid(true) {
        if is_ready(true) {
            println!("Function calls in conditions");
        }
    }
}
"#;

const COLLAPSIBLE_IF_WITH_SIMPLE_NUMERICAL_CONDITIONS: &str = r#"
fn main() {
    let a = 10_u32;
    let b = 20_u32;
    let c = 5_u32;

    if a > b {
        if c < b {
            println!("Simple numerical conditions");
        }
    }
}
"#;

const COLLAPSIBLE_IF_WITH_ELSE_CLAUSE: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            println!("Hello");
        } else {
            println!("World");
        }
    }
}
"#;

const COLLAPSIBLE_IF_WITH_ELSE_ON_OUTER_IF: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            println!("Hello");
        } 
    } else {
        println!("World");
    }
}
"#;

const COLLAPSIBLE_IF_WITH_INDEPENDENT_STATEMENT: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            println!("Hello");
        }
        println!("World");
    }
}
"#;

#[test]
fn collapsible_if_in_boolean_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS, @r#"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:7:5
       |
     7 | /     if x || z {
     8 | |         if y && z {
     9 | |             println!("Hello");
    10 | |         }
    11 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn collapsible_if_in_boolean_conditions_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;
        if (x || z) && (y && z) {
            println!("Hello");
        }
    }
    "#);
}

#[test]
fn collapsible_if_in_boolean_conditions_with_comment_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_WITH_COMMENT, @r"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:7:5
       |
     7 | /     if x || z {
     8 | |         if y && z {
    ...  |
    11 | |         }
    12 | |     }
       | |_____-
       |
    ");
}

#[test]
fn collapsible_if_in_boolean_conditions_with_comment_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_WITH_COMMENT, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;
        if (x || z) && (y && z) {
            // Just a comment.
            println!("Hello");
        }
    }
    "#);
}

#[test]
fn collapsible_if_in_boolean_conditions_allowed_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_ALLOWED, @r#"
    "#);
}

#[test]
fn collapsible_if_in_boolean_conditions_allowed_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_ALLOWED, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;

        #[allow(collapsible_if)]
        if x || z {
            if y && z {
                println!("Hello");
            }
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_combinable_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_COMBINABLE_CONDITIONS, @r#"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:6:5
       |
     6 | /     if x {
     7 | |         if z {
     8 | |             println!("No fix here");
     9 | |         }
    10 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn collapsible_if_with_combinable_conditions_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_COMBINABLE_CONDITIONS, @r#"
    fn main() {
        let x = true;
        let z = true;
        if (x) && (z) {
            println!("No fix here");
        }
    }
    "#);
}

#[test]
fn collapsible_if_in_conditions_with_complex_expressions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_CONDITIONS_WITH_COMPLEX_EXPRESSIONS, @r#"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:10:5
       |
    10 | /     if x + y > a {
    11 | |         if z * b < c {
    12 | |             println!("Complex conditions");
    13 | |         }
    14 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn collapsible_if_in_conditions_with_complex_expressions_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_IN_CONDITIONS_WITH_COMPLEX_EXPRESSIONS, @r#"
    fn main() {
        let x = 3_u32;
        let y = 4_u32;
        let z = 5_u32;
        let a = 5_u32;
        let b = 2_u32;
        let c = 10_u32;
        if (x + y > a) && (z * b < c) {
            println!("Complex conditions");
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_function_calls_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_FUNCTION_CALLS, @r#"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:6:5
       |
     6 | /     if is_valid(true) {
     7 | |         if is_ready(true) {
     8 | |             println!("Function calls in conditions");
     9 | |         }
    10 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn collapsible_if_with_function_calls_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_FUNCTION_CALLS, @r#"
    fn is_valid(_a: bool) -> bool { true } 
    fn is_ready(_b: bool) -> bool { true } 

    fn main() {
        if (is_valid(true)) && (is_ready(true)) {
            println!("Function calls in conditions");
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_simple_numerical_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_SIMPLE_NUMERICAL_CONDITIONS, @r#"
    warning: Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
      --> lib.cairo:7:5
       |
     7 | /     if a > b {
     8 | |         if c < b {
     9 | |             println!("Simple numerical conditions");
    10 | |         }
    11 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn collapsible_if_with_simple_numerical_conditions_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_SIMPLE_NUMERICAL_CONDITIONS, @r#"
    fn main() {
        let a = 10_u32;
        let b = 20_u32;
        let c = 5_u32;
        if (a > b) && (c < b) {
            println!("Simple numerical conditions");
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_else_clause_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_ELSE_CLAUSE, @r#"
    "#);
}

#[test]
fn collapsible_if_with_else_clause_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_ELSE_CLAUSE, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;

        if x || z {
            if y && z {
                println!("Hello");
            } else {
                println!("World");
            }
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_else_on_outer_if_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_ELSE_ON_OUTER_IF, @r#"
    "#);
}

#[test]
fn collapsible_if_with_else_on_outer_if_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_ELSE_ON_OUTER_IF, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;

        if x || z {
            if y && z {
                println!("Hello");
            } 
        } else {
            println!("World");
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_independent_statement_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_INDEPENDENT_STATEMENT, @r#"
    "#);
}

#[test]
fn collapsible_if_with_independent_statement_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_INDEPENDENT_STATEMENT, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;

        if x || z {
            if y && z {
                println!("Hello");
            }
            println!("World");
        }
    }
    "#);
}
