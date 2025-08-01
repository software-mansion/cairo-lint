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

const COLLAPSIBLE_IF_WITH_MULTIPLE_CONDITIONS: &str = r#"
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if (x || z) && (y && z) {
        if (y && z) && (x || z) {
            println!("Hello");
        }
    }
}
"#;

const IF_LET_TO_IGNORE_WITH_ASSERT: &str = r#"
fn main() {
    let x = Option::Some(true);
    let y = Option::Some(true);

    if let Option::Some(_z) = x {
        assert!(x == y);
    }
}
"#;

const COLLAPSIBLE_IFS_INSIDE_IF_LET: &str = r#"
fn main() {
    let x = Option::Some(true);

    let a = true;
    let b = true;
    let c = false;

    if let Option::Some(_y) = x {
         if a || b {
            if b && c {
                println!("Hello");
            }
        }
    }
}
"#;

const COLLAPSIBLE_IF_IN_TRAIT: &str = r#"
#[derive(Drop)]
struct MyStruct {
    x: bool,
    y: bool,
}

trait TExample {
    fn check_conditions(self: @MyStruct, z: bool) {
        if *self.x {
            if *self.y && z {
                println!("Trait default function with collapsible if");
            }
        }
    }
}

impl Example of TExample {}

fn main() {
    let instance = MyStruct { x: true, y: true };
    instance.check_conditions(true);
}
"#;

const SIMPLE_IF_INSIDE_IF_LET: &str = r#"
fn main() {
    let x = Option::Some(true);

    let a = true;
    let b = true;

    if let Option::Some(_y) = x {
        if a || b {
            println!("Hello");
        }
    }
}
"#;

const NON_COLLAPSIBLE_IF_LETS: &str = r#"
fn main() {
    let x = Some(Some(42));
    let y = Option::<u8>::None;

    if let Some(_) = x {
        if let Some(value) = y {
            println!("The value is: {}", value);
        }
    }
}
"#;

const IF_WITH_ASSERT: &str = r#"
fn main() {
    let x = Some(42);
    let y = Some(2);
    let z = Some(10);

    if x == y {
        assert!(z == Some(42));
    }
}
"#;

const IF_LET_NESTED_WITHIN_IF: &str = r#"
fn main() {
    let x = Some(42);
    let y = Some(2);

    if x == y {
        if let Some(z) = x {
            println!("Hello, {}", z);
        }
    }
}
"#;

const IF_LET_WITH_ASSERT: &str = r#"
fn main() {
    let x = Some(42);

    if let Some(y) = x {
        assert!(y == 42);
    }
}
"#;

#[test]
fn collapsible_if_in_boolean_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:7:5-11:5
          if x || z {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:7:5-12:5
          if x || z {
     _____^
    | ...
    |     }
    |_____^
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_BOOLEAN_CONDITIONS_ALLOWED, @"");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_COMBINABLE_CONDITIONS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:6:5-10:5
          if x {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_CONDITIONS_WITH_COMPLEX_EXPRESSIONS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:10:5-14:5
          if x + y > a {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_FUNCTION_CALLS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:6:5-10:5
          if is_valid(true) {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn collapsible_if_with_function_calls_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_FUNCTION_CALLS, @r#"
    fn is_valid(_a: bool) -> bool {
        true
    }
    fn is_ready(_b: bool) -> bool {
        true
    }

    fn main() {
        if (is_valid(true)) && (is_ready(true)) {
            println!("Function calls in conditions");
        }
    }
    "#);
}

#[test]
fn collapsible_if_with_simple_numerical_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_SIMPLE_NUMERICAL_CONDITIONS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:7:5-11:5
          if a > b {
     _____^
    | ...
    |     }
    |_____^
    ");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_ELSE_CLAUSE, @"");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_ELSE_ON_OUTER_IF, @"");
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
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_INDEPENDENT_STATEMENT, @"");
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

#[test]
fn collapsible_if_with_multiple_conditions_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_WITH_MULTIPLE_CONDITIONS, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:7:5-11:5
          if (x || z) && (y && z) {
     _____^
    | ...
    |     }
    |_____^
    ");
}

#[test]
fn collapsible_if_with_multiple_conditions_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_WITH_MULTIPLE_CONDITIONS, @r#"
    fn main() {
        let x = true;
        let y = true;
        let z = false;
        if ((x || z) && (y && z)) && ((y && z) && (x || z)) {
            println!("Hello");
        }
    }
    "#);
}

#[test]
fn if_let_to_ignore_with_assert_diagnostic() {
    test_lint_diagnostics!(IF_LET_TO_IGNORE_WITH_ASSERT, @"");
}

#[test]
fn if_let_to_ignore_fixer() {
    test_lint_fixer!(IF_LET_TO_IGNORE_WITH_ASSERT, @r"
    fn main() {
        let x = Option::Some(true);
        let y = Option::Some(true);

        if let Option::Some(_z) = x {
            assert!(x == y);
        }
    }
    ");
}

#[test]
fn if_with_assert_diagnostic() {
    test_lint_diagnostics!(IF_WITH_ASSERT, @"");
}

#[test]
fn if_with_assert_fixer() {
    test_lint_fixer!(IF_WITH_ASSERT, @r"
    fn main() {
        let x = Some(42);
        let y = Some(2);
        let z = Some(10);

        if x == y {
            assert!(z == Some(42));
        }
    }
    ");
}

#[test]
fn if_let_nested_within_if_diagnostics() {
    test_lint_diagnostics!(IF_LET_NESTED_WITHIN_IF, @"");
}

#[test]
fn if_let_nested_within_if_fixer() {
    test_lint_fixer!(IF_LET_NESTED_WITHIN_IF, @r#"
    fn main() {
        let x = Some(42);
        let y = Some(2);

        if x == y {
            if let Some(z) = x {
                println!("Hello, {}", z);
            }
        }
    }
    "#);
}

#[test]
fn collapsible_if_in_trait_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IF_IN_TRAIT, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:10:9-14:9
              if *self.x {
     _________^
    | ...
    |         }
    |_________^
    ");
}

#[test]
fn collapsible_if_in_trait_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IF_IN_TRAIT, @r#"
    #[derive(Drop)]
    struct MyStruct {
        x: bool,
        y: bool,
    }

    trait TExample {
        fn check_conditions(
            self: @MyStruct, z: bool,
        ) {
            if (*self.x) && (*self.y && z) {
                println!("Trait default function with collapsible if");
            }
        }
    }

    impl Example of TExample {}

    fn main() {
        let instance = MyStruct { x: true, y: true };
        instance.check_conditions(true);
    }
    "#);
}

#[test]
fn collapsible_ifs_inside_if_let_diagnostics() {
    test_lint_diagnostics!(COLLAPSIBLE_IFS_INSIDE_IF_LET, @r"
    Plugin diagnostic: Each `if`-statement adds one level of nesting, which makes code look more complex than it really is.
     --> lib.cairo:10:10-14:9
               if a || b {
     __________^
    | ...
    |         }
    |_________^
    ");
}

#[test]
fn collapsible_ifs_inside_if_let_fixer() {
    test_lint_fixer!(COLLAPSIBLE_IFS_INSIDE_IF_LET, @r#"
    fn main() {
        let x = Option::Some(true);

        let a = true;
        let b = true;
        let c = false;

        if let Option::Some(_y) = x {
            if (a || b) && (b && c) {
                println!("Hello");
            }
        }
    }
    "#);
}

#[test]
fn non_collapsible_if_lets_diagnostics() {
    test_lint_diagnostics!(NON_COLLAPSIBLE_IF_LETS, @r"");
}

#[test]
fn non_collapsible_if_lets_fixer() {
    test_lint_fixer!(NON_COLLAPSIBLE_IF_LETS, @r#"
    fn main() {
        let x = Some(Some(42));
        let y = Option::<u8>::None;

        if let Some(_) = x {
            if let Some(value) = y {
                println!("The value is: {}", value);
            }
        }
    }
    "#);
}

#[test]
fn simple_if_inside_if_let_diagnostics() {
    test_lint_diagnostics!(SIMPLE_IF_INSIDE_IF_LET, @r"");
}

#[test]
fn if_let_with_assert_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_ASSERT, @r"")
}
