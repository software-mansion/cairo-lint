use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_BREAK: &str = r#"
fn main() {
   loop {
       break ();
   }
}
"#;

const SIMPLE_BREAK_ALLOWED: &str = r#"
fn main() {
   loop {
       #[allow(break_unit)]
       break ();
   }
}
"#;

const BREAK_INSIDE_OF_IF: &str = r#"
fn main() {
    let mut a = 1_u32;
    #[allow(loop_for_while)]
    loop {
        if a == 10 {
            break ();
        }
        a += 1;
    }
}
"#;

const BREAK_INSIDE_OF_IF_WITH_COMMENT: &str = r#"
fn main() {
    let mut a = 1_u32;
    #[allow(loop_for_while)]
    loop {
        if a == 10 {
            // this is a break
            break ();
            // this was a break
        }
        a += 1;
    }
}
"#;

#[test]
fn simple_break_diagnostics() {
    test_lint_diagnostics!(SIMPLE_BREAK, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.
     --> lib.cairo:4:8
      |
    4 |        break ();
      |        ---------
      |
    ");
}

#[test]
fn simple_break_fixer() {
    test_lint_fixer!(SIMPLE_BREAK, @r#"
    fn main() {
       loop {
           break;
       }
    }
    "#);
}

#[test]
fn simple_break_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_BREAK_ALLOWED, @"");
}

#[test]
fn simple_break_allowed_fixer() {
    test_lint_fixer!(SIMPLE_BREAK_ALLOWED, @r"
    fn main() {
       loop {
           #[allow(break_unit)]
           break ();
       }
    }
    ");
}

#[test]
fn break_inside_of_if_diagnostics() {
    test_lint_diagnostics!(BREAK_INSIDE_OF_IF, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.
     --> lib.cairo:7:13
      |
    7 |             break ();
      |             ---------
      |
    ");
}

#[test]
fn break_inside_of_if_fixer() {
    test_lint_fixer!(BREAK_INSIDE_OF_IF, @r#"
    fn main() {
        let mut a = 1_u32;
        #[allow(loop_for_while)]
        loop {
            if a == 10 {
                break;
            }
            a += 1;
        }
    }
    "#);
}

#[test]
fn break_inside_of_if_with_comment_diagnostics() {
    test_lint_diagnostics!(BREAK_INSIDE_OF_IF_WITH_COMMENT, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.
     --> lib.cairo:8:13
      |
    8 |             break ();
      |             ---------
      |
    ");
}

#[test]
fn break_inside_of_if_with_comment_fixer() {
    test_lint_fixer!(BREAK_INSIDE_OF_IF_WITH_COMMENT, @r#"
    fn main() {
        let mut a = 1_u32;
        #[allow(loop_for_while)]
        loop {
            if a == 10 {
                // this is a break
                break;
                // this was a break
            }
            a += 1;
        }
    }
    "#);
}
