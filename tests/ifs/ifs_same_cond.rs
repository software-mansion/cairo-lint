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

const SAME_CONDITION_WITH_ELSE_WITH_COMMENT: &str = r#"
fn main() {
    let a = 1;
    let b = 1;
    if a == b {
        // Just a comment.
        println!("a is equal to b");
    } else if a == b {
        // Just a comment 2.
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
    test_lint_diagnostics!(SAME_CONDITION_WITH_ELSE, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5-9:5
          if a == b {
     _____^
    | ...
    |     }
    |_____^
    ");
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
fn same_condition_with_else_with_comment_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_ELSE_WITH_COMMENT, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5-11:5
          if a == b {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn same_condition_with_else_with_comment_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_ELSE_WITH_COMMENT, @r#"
    fn main() {
        let a = 1;
        let b = 1;
        if a == b {
            // Just a comment.
            println!("a is equal to b");
        } else if a == b {
            // Just a comment 2.
            println!("a is equal to b");
        }
    }
    "#);
}

#[test]
fn same_condition_with_boolean_diagnostics() {
    test_lint_diagnostics!(SAME_CONDITION_WITH_BOOLEAN, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5-9:5
          if condition {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    test_lint_diagnostics!(SAME_CONDITION_WITH_FELT252, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:6:5-10:5
          if str1 == str2 {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn same_condition_with_felt252_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_FELT252, @r#"
    fn main() {
        let str1: felt252 = 'hello';
        let str2: felt252 = 'hello';

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
    test_lint_diagnostics!(SAME_CONDITION_WITH_STRUCT, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:11:5-15:5
          if p1.x == p2.x && p1.y == p2.y {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    test_lint_diagnostics!(SAME_CONDITION_WITH_MULTIPLE_IF_ELSE, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:12:12-17:5
          } else if str1 == str2 {
     ____________^
    | ...
    |     } 
    |_____^
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:10:12-17:5
          } else if str1 == str2 {
     ____________^
    | ...
    |     } 
    |_____^
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:8:12-17:5
          } else if str1 == str2 {
     ____________^
    | ...
    |     } 
    |_____^
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:6:5-17:5
          if str1 == str2 {
     _____^
    | ...
    |     } 
    |_____^
    ");
}

#[test]
fn same_condition_with_multiple_if_else_fixer() {
    test_lint_fixer!(SAME_CONDITION_WITH_MULTIPLE_IF_ELSE, @r#"
    fn main() {
        let str1: felt252 = 'hello';
        let str2: felt252 = 'hello';

        if str1 == str2 {
            println!("Strings are equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } else if str1 == str2 {
            println!("Strings are still equal");
        } else if str1 == str2 {
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
        let a: u32 = 1;
        let b: u32 = 2;

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
    test_lint_diagnostics!(COMBINED_CONDITIONS_WITH_DIFFERENT_IF, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:7:5-13:5
          if x == z {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn combined_conditions_with_different_if_fixer() {
    test_lint_fixer!(COMBINED_CONDITIONS_WITH_DIFFERENT_IF, @r#"
    fn main() {
        let x: u32 = 5;
        let y: u32 = 10;
        let z: u32 = 5;

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
    test_lint_diagnostics!(IF_WITH_FUNCTIONS, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:8:4-12:5
         if foo() {
     ____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn if_with_functions_fixer() {
    test_lint_fixer!(IF_WITH_FUNCTIONS, @r#"
    fn foo() -> bool {
        println!("foo");
        true
    }

    fn main() {
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
    test_lint_diagnostics!(GREATER_LESSER_COMPARISON, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5-9:5
          if a > 3 {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn greater_lesser_comparison_fixer() {
    test_lint_fixer!(GREATER_LESSER_COMPARISON, @r#"
    fn main() {
        let a: u32 = 3;

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
    test_lint_diagnostics!(SAME_CONDITIONS_WITH_LITERALS_AND_VARS, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:5:5-9:5
          if a == 3 {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn same_conditions_with_literals_and_vars_fixer() {
    test_lint_fixer!(SAME_CONDITIONS_WITH_LITERALS_AND_VARS, @r#"
    fn main() {
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
    test_lint_diagnostics!(SAME_CONDITIONS_WITH_LITERALS, @r"
    Plugin diagnostic: Consecutive `if` with the same condition found.
     --> lib.cairo:4:5-8:5
          if 2 == 3 {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn same_conditions_with_literals_fixer() {
    test_lint_fixer!(SAME_CONDITIONS_WITH_LITERALS, @r#"
    fn main() {
        if 2 == 3 {
            println!("a == 3");
        } else if 2 == 3 {
            println!("a == 3");
        }
    }
    "#);
}
