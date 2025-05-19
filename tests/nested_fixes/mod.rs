use crate::{test_lint_diagnostics, test_lint_fixer};

const NESTED_IFS: &str = r#"
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

#[test]
fn nested_ifs_diagnostics() {
    test_lint_diagnostics!(NESTED_IFS, @r"
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
fn nested_ifs_fixer() {
    test_lint_fixer!(NESTED_IFS, @r#"
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
