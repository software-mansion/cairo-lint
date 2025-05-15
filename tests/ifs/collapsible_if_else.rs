use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_ELSE_IF_WITH_NEW_LINE: &str = r#"
fn main() {
    let x = true;
    if x {
        println!("x is true");
    }
    else {
        if !x {
            println!("x is false");
        }
    }
}
"#;

const SIMPLE_ELSE_IF_WITH_NEW_LINE_ALLOWED: &str = r#"
fn main() {
    let x = true;
    #[allow(collapsible_if_else)]
    if x {
        println!("x is true");
    }
    else {
        if !x {
            println!("x is false");
        }
    }
}
"#;

const SIMPLE_ELSE_IF_WITHOUT_NEW_LINE: &str = r#"
fn main() {
    let x = true;
    if x {
        println!("x is true");
    } else {
        if !x {
            println!("x is false");
        }
    }
}
"#;

const MULTIPLE_ELSE_IF: &str = r#"
fn main() {
    let x = true;
    if x {
        println!("x is true");
    }
    else {
        if !x {
            println!("x is false");
        }
        else {
            println!("x is neither true nor false");
        }
    }
}
"#;

const ELSE_IF_WITH_MULTIPLE_STATEMENTS: &str = r#"
fn main() {
    let x = true;
    if x {
        println!("x is true");
    }
    else {
        if !x {
            println!("x is false");
        }
        else {
            let y = 10;
            println!("y is {}", y);
        }
    }
}
"#;

const ELSE_IF_INSIDE_LOOP: &str = r#"
fn main() {
    let mut a = 1_u32;
    loop {
        if a == 10 {
            a += 1;
        } else {
            if a == 15 {
                break;
            }
            else {
                a += 2;
            }
        }
    }
}
"#;

#[test]
fn simple_else_if_with_new_line_diagnostics() {
    test_lint_diagnostics!(SIMPLE_ELSE_IF_WITH_NEW_LINE, @r"
    Plugin diagnostic: Consider using else if instead of else { if ... }
     --> lib.cairo:4:5-11:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn simple_else_if_with_new_line_fixer() {
    test_lint_fixer!(SIMPLE_ELSE_IF_WITH_NEW_LINE, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        }
        else if !x  {
                println!("x is false");
            }
     }
    "#);
}

#[test]
fn simple_else_if_with_new_line_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_ELSE_IF_WITH_NEW_LINE_ALLOWED, @r#"
    "#);
}

#[test]
fn simple_else_if_with_new_line_allowed_fixer() {
    test_lint_fixer!(SIMPLE_ELSE_IF_WITH_NEW_LINE_ALLOWED, @r#"
    fn main() {
        let x = true;
        #[allow(collapsible_if_else)]
        if x {
            println!("x is true");
        }
        else {
            if !x {
                println!("x is false");
            }
        }
    }
    "#);
}

#[test]
fn simple_else_if_without_new_line_diagnostics() {
    test_lint_diagnostics!(SIMPLE_ELSE_IF_WITHOUT_NEW_LINE, @r"
    Plugin diagnostic: Consider using else if instead of else { if ... }
     --> lib.cairo:4:5-10:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn simple_else_if_without_new_line_fixer() {
    test_lint_fixer!(SIMPLE_ELSE_IF_WITHOUT_NEW_LINE, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        } else if !x  {
                println!("x is false");
            }
     }
    "#);
}

#[test]
fn multiple_else_if_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_ELSE_IF, @r"
    Plugin diagnostic: Consider using else if instead of else { if ... }
     --> lib.cairo:4:5-14:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn multiple_else_if_fixer() {
    test_lint_fixer!(MULTIPLE_ELSE_IF, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        }
        else if !x  {
                println!("x is false");
            }
             else {
                println!("x is neither true nor false");
            }
    }
    "#);
}

#[test]
fn else_if_with_multiple_statements_diagnostics() {
    test_lint_diagnostics!(ELSE_IF_WITH_MULTIPLE_STATEMENTS, @r"
    Plugin diagnostic: Consider using else if instead of else { if ... }
     --> lib.cairo:4:5-15:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn else_if_with_multiple_statements_fixer() {
    test_lint_fixer!(ELSE_IF_WITH_MULTIPLE_STATEMENTS, @r#"
    fn main() {
        let x = true;
        if x {
            println!("x is true");
        }
        else if !x  {
                println!("x is false");
            }
             else {
                let y = 10;
                println!("y is {}", y);
            }
    }
    "#);
}

#[test]
fn else_if_inside_loop_diagnostics() {
    test_lint_diagnostics!(ELSE_IF_INSIDE_LOOP, @r"
    Plugin diagnostic: Consider using else if instead of else { if ... }
     --> lib.cairo:5:9-14:9
              if a == 10 {
     _________^
    | ...
    |         }
    |_________^
    ");
}

#[test]
fn else_if_inside_loop_fixer() {
    test_lint_fixer!(ELSE_IF_INSIDE_LOOP, @r#"
    fn main() {
        let mut a = 1_u32;
        loop {
            if a == 10 {
                a += 1;
            } else if a == 15  {
                    break;
                }
                 else {
                    a += 2;
                }
        }
    }
    "#);
}

const A: &str = r#"
fn main() {
    let x = true;
    let a = true;
    let b = true;
    let c = false;

    if x {
         if a || b {
            if b && c {
                println!("Hello");
            }
        }
    }
}
"#;

const B: &str = r#"
fn main() {
    let x = true;
    let a = true;
    let b = true;
    let c = false;

    if x {
        if (a || b) && (b && c) {
            println!("Hello");
        }
    }
}
"#;

#[test]
fn b_fixer() {
    test_lint_fixer!(B, @r#"
    fn main() {
        let x = true;
        let a = true;
        let b = true;
        let c = false;
        if (x) && ((a || b) && (b && c)) {
            println!("Hello");
        }
    }
    "#);
}

#[test]
fn a_diagnostics() {
    test_lint_diagnostics!(A, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:9:10-13:9
               if a || b {
     __________^
    | ...
    |         }
    |_________^
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:8:5-14:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn a_fixer() {
    test_lint_fixer!(A, @r#"
    fn main() {
        let x = true;
        let a = true;
        let b = true;
        let c = false;
        if (x) && ((a || b) && (b && c)) {
            println!("Hello");
        }
    }
    "#);
}
