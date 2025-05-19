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

const NESTED_DESTRUCTURING_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(Option::Some(1_felt252));
    match variable {
        Option::Some(a) => match a {
            Option::Some(b) => println!("{b}"),
            _ => (),
        },
        _ => (),
    };
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

#[test]
fn nested_destructuring_match_diagnostics() {
    test_lint_diagnostics!(NESTED_DESTRUCTURING_MATCH, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:5:28-8:9
              Option::Some(a) => match a {
     ____________________________^
    | ...
    |         },
    |_________^
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5-10:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn nested_destructuring_match_fixer() {
    test_lint_fixer!(NESTED_DESTRUCTURING_MATCH, @r#"
    fn main() {
        let variable = Option::Some(Option::Some(1_felt252));
        if let Option::Some(a) = variable {
            if let Option::Some(b) = a {
                println!("{b}")
            }
        };
    }
    "#);
}
