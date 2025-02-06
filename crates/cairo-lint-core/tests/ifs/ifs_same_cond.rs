use crate::{test_lint_diagnostics, test_lint_fixer};

const SAME_CONDITION_WITH_ELSE: &str = r#"
fn main() {
    let a = 1;
    let b = 1;
    if a == b {
        println!("a is equal to b");
    } else if a == b {
        println!("a is equal to b");
    }
}
"#;

const SAME_CONDITION_WITH_BOOLEAN: &str = r#"
fn main() {
    let condition = true;

    if condition {
        println!("Condition is true");
    } else if condition {
        println!("Condition is still true");
    }
}
"#;

const SAME_CONDITION_WITH_FELT252: &str = r#"
fn main(){
    let str1:felt252 = 'hello';
    let str2:felt252 = 'hello';

    if str1 == str2 {
        println!("Strings are equal");
    } else if str1 == str2 {
        println!("Strings are still equal");
    }
}
"#;

const SAME_CONDITION_WITH_STRUCT: &str = r#"
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = Point { x: 3, y: 4 };

    if p1.x == p2.x && p1.y == p2.y {
        println!("Points are equal");
    } else if p1.x == p2.x && p1.y == p2.y {
        println!("Points are still equal");
    }
}
"#;

const SAME_CONDITION_WITH_MULTIPLE_IF_ELSE: &str = r#"
fn main(){
    let str1:felt252 = 'hello';
    let str2:felt252 = 'hello';

    if str1 == str2 {
        println!("Strings are equal");
    } else if str1 == str2 {
        println!("Strings are still equal");
    } else if str1 == str2 {
        println!("Strings are still equal");
    } else if str1 == str2 {
        println!("Strings are still equal");
    } 
    else if str1 == str2 {
        println!("Strings are still equal");
    } 
}
"#;

const SIMILAR_CONDITIONS: &str = r#"
fn main() {
    let a:u32 = 1;
    let b:u32 = 2;

    if a == b {
        println!("a is equal to b");
    } else if a < b {
        println!("a is less than b");
    } else if a > b {
        println!("a is greater than b");
    }
}
"#;

const COMBINED_CONDITIONS_WITH_DIFFERENT_IF: &str = r#"
fn main() {
    let x:u32 = 5;
    let y:u32 = 10;
    let z:u32 = 5;

    if x == z {
        println!("x is equal to z");
    } else if x == z {
        println!("x is still equal to z");
    } else if x + 5 == y {
        println!("x plus 5 is equal to y");
    }
}
"#;

const IF_WITH_FUNCTIONS: &str = r#"
fn foo() -> bool{
        println!("foo");
        true
    }

fn main(){
   if foo() {
        println!("foo");
    } else if foo() { 
        println!("foo");
    }
}
"#;

const GREATER_LESSER_COMPARISON: &str = r#"
fn main(){
    let a:u32 = 3;

    if a > 3 {
        println!("a == 3");
    } else if a > 3 {
        println!("3 == a");
    }
}
"#;

const SAME_CONDITIONS_WITH_LITERALS_AND_VARS: &str = r#"
fn main(){
    let a = 3;

    if a == 3 {
        println!("a == 3");
    } else if a == 3 {
        println!("a == 3");
    }
}
"#;

const SAME_CONDITIONS_WITH_LITERALS: &str = r#"
fn main(){

    if 2 == 3 {
        println!("a == 3");
    } else if 2 == 3 {
        println!("a == 3");
    }
}
"#;

#[test]
fn same_condition_with_else_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_ELSE, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5
      |
    5 | /     if a == b {
    6 | |         println!("a is equal to b");
    7 | |     } else if a == b {
    8 | |         println!("a is equal to b");
    9 | |     }
      | |_____-
      |
    "#);
}

#[test]
fn same_condition_with_else_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_ELSE, @r#"
    fn main() {
        let a = 1;
        let b = 1;
        if a == b {
            println!("a is equal to b");
        } else if a == b {
            println!("a is equal to b");
        }
    }
    "#);
}

#[test]
fn same_condition_with_boolean_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_BOOLEAN, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5
      |
    5 | /     if condition {
    6 | |         println!("Condition is true");
    7 | |     } else if condition {
    8 | |         println!("Condition is still true");
    9 | |     }
      | |_____-
      |
    "#);
}

#[test]
fn same_condition_with_boolean_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_BOOLEAN, @r#"
    fn main() {
        let condition = true;

        if condition {
            println!("Condition is true");
        } else if condition {
            println!("Condition is still true");
        }
    }
    "#);
}

#[test]
fn same_condition_with_felt252_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_FELT252, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:6:5
       |
     6 | /     if str1 == str2 {
     7 | |         println!("Strings are equal");
     8 | |     } else if str1 == str2 {
     9 | |         println!("Strings are still equal");
    10 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn same_condition_with_felt252_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_FELT252, @r#"
    fn main(){
        let str1:felt252 = 'hello';
        let str2:felt252 = 'hello';

        if str1 == str2 {
            println!("Strings are equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        }
    }
    "#);
}

#[test]
fn same_condition_with_struct_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_STRUCT, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:11:5
       |
    11 | /     if p1.x == p2.x && p1.y == p2.y {
    12 | |         println!("Points are equal");
    13 | |     } else if p1.x == p2.x && p1.y == p2.y {
    14 | |         println!("Points are still equal");
    15 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn same_condition_with_struct_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_STRUCT, @r#"
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let p1 = Point { x: 3, y: 4 };
        let p2 = Point { x: 3, y: 4 };

        if p1.x == p2.x && p1.y == p2.y {
            println!("Points are equal");
        } else if p1.x == p2.x && p1.y == p2.y {
            println!("Points are still equal");
        }
    }
    "#);
}

#[test]
fn same_condition_with_multiple_if_else_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_MULTIPLE_IF_ELSE, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:12:12
       |
    12 |       } else if str1 == str2 {
       |  ____________-
    13 | |         println!("Strings are still equal");
    ...  |
    16 | |         println!("Strings are still equal");
    17 | |     } 
       | |_____-
       |
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:10:12
       |
    10 |       } else if str1 == str2 {
       |  ____________-
    11 | |         println!("Strings are still equal");
    ...  |
    16 | |         println!("Strings are still equal");
    17 | |     } 
       | |_____-
       |
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:8:12
       |
     8 |       } else if str1 == str2 {
       |  ____________-
     9 | |         println!("Strings are still equal");
    ...  |
    16 | |         println!("Strings are still equal");
    17 | |     } 
       | |_____-
       |
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:6:5
       |
     6 | /     if str1 == str2 {
     7 | |         println!("Strings are equal");
    ...  |
    16 | |         println!("Strings are still equal");
    17 | |     } 
       | |_____-
       |
    "#);
}

#[test]
fn same_condition_with_multiple_if_else_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_MULTIPLE_IF_ELSE, @r#"
    fn main(){
        let str1:felt252 = 'hello';
        let str2:felt252 = 'hello';

        if str1 == str2 {
            println!("Strings are equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } 
        else if str1 == str2 {
            println!("Strings are still equal");
        } 
    }
    "#);
}

#[test]
fn similar_conditions_diagnostics() {
    test_lint_diagnostics!(SIMILAR_CONDITIONS, @r#"
    "#);
}

#[test]
fn similar_conditions_fixer() {
    test_lint_fixer!(SIMILAR_CONDITIONS, @r#"
    fn main() {
        let a:u32 = 1;
        let b:u32 = 2;

        if a == b {
            println!("a is equal to b");
        } else if a < b {
            println!("a is less than b");
        } else if a > b {
            println!("a is greater than b");
        }
    }
    "#);
}

#[test]
fn combined_conditions_with_different_if_diagnostics() {
    test_lint_diagnostics!(COMBINED_CONDITIONS_WITH_DIFFERENT_IF, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:7:5
       |
     7 | /     if x == z {
     8 | |         println!("x is equal to z");
    ...  |
    12 | |         println!("x plus 5 is equal to y");
    13 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn combined_conditions_with_different_if_fixer() {
    test_lint_fixer!(COMBINED_CONDITIONS_WITH_DIFFERENT_IF, @r#"
    fn main() {
        let x:u32 = 5;
        let y:u32 = 10;
        let z:u32 = 5;

        if x == z {
            println!("x is equal to z");
        } else if x == z {
            println!("x is still equal to z");
        } else if x + 5 == y {
            println!("x plus 5 is equal to y");
        }
    }
    "#);
}

#[test]
fn if_with_functions_diagnostics() {
    test_lint_diagnostics!(IF_WITH_FUNCTIONS, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
      --> lib.cairo:8:4
       |
     8 | /    if foo() {
     9 | |         println!("foo");
    10 | |     } else if foo() { 
    11 | |         println!("foo");
    12 | |     }
       | |_____-
       |
    "#);
}

#[test]
fn if_with_functions_fixer() {
    test_lint_fixer!(IF_WITH_FUNCTIONS, @r#"
    fn foo() -> bool{
            println!("foo");
            true
        }

    fn main(){
       if foo() {
            println!("foo");
        } else if foo() { 
            println!("foo");
        }
    }
    "#);
}

#[test]
fn greater_lesser_comparison_diagnostics() {
    test_lint_diagnostics!(GREATER_LESSER_COMPARISON, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5
      |
    5 | /     if a > 3 {
    6 | |         println!("a == 3");
    7 | |     } else if a > 3 {
    8 | |         println!("3 == a");
    9 | |     }
      | |_____-
      |
    "#);
}

#[test]
fn greater_lesser_comparison_fixer() {
    test_lint_fixer!(GREATER_LESSER_COMPARISON, @r#"
    fn main(){
        let a:u32 = 3;

        if a > 3 {
            println!("a == 3");
        } else if a > 3 {
            println!("3 == a");
        }
    }
    "#);
}

#[test]
fn same_conditions_with_literals_and_vars_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITIONS_WITH_LITERALS_AND_VARS, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5
      |
    5 | /     if a == 3 {
    6 | |         println!("a == 3");
    7 | |     } else if a == 3 {
    8 | |         println!("a == 3");
    9 | |     }
      | |_____-
      |
    "#);
}

#[test]
fn same_conditions_with_literals_and_vars_fixer() {
    test_lint_fixer!(SAME_CONDITIONS_WITH_LITERALS_AND_VARS, @r#"
    fn main(){
        let a = 3;

        if a == 3 {
            println!("a == 3");
        } else if a == 3 {
            println!("a == 3");
        }
    }
    "#);
}

#[test]
fn same_conditions_with_literals_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITIONS_WITH_LITERALS, @r#"
    warning: Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:4:5
      |
    4 | /     if 2 == 3 {
    5 | |         println!("a == 3");
    6 | |     } else if 2 == 3 {
    7 | |         println!("a == 3");
    8 | |     }
      | |_____-
      |
    "#);
}

#[test]
fn same_conditions_with_literals_fixer() {
    test_lint_fixer!(SAME_CONDITIONS_WITH_LITERALS, @r#"
    fn main(){

        if 2 == 3 {
            println!("a == 3");
        } else if 2 == 3 {
            println!("a == 3");
        }
    }
    "#);
}
