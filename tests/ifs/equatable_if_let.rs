use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_EQUALITY_CASES_OK: &str = r#"
fn main() {
    let x = Option::Some(1_felt252);
    if x == Option::Some(1_felt252) {
        println!("Value");
    }
}
"#;

const COMPLEX_EQUALITY_DESTRUCTURING_IF_LET: &str = r#"
fn main() {
    let x = Option::Some(1_felt252);
    let y = Option::Some(1_felt252);
    if let (Option::Some(a), Option::Some(b)) = (x, y) {
        let _avoid_collapsible = 1_u32;
        if a == b {
            println!("{a}");
        }
    }
}
"#;

const SIMPLE_VALUE_PATTERN_MATCHING: &str = r#"
fn main() {
    let a = 2;
    if let 2 = a {}
}
"#;

const SIMPLE_VALUE_PATTERN_MATCHING_WITH_COMMENT: &str = r#"
fn main() {
    let a = 2;
    if let 2 = a {
      // Just a comment.
    }
}
"#;

const ENUM_UNIT_VARIANT_PATTERN_MATCHING: &str = r#"
enum Enum {
    UnitVariant
} 

fn main() {
    let e = Enum::UnitVariant;

    if let Enum::UnitVariant = e {}
}
"#;

const COMPLEX_EQUALITY_DESTRUCTURING: &str = r#"
fn main() {
    let x = Option::Some(1_felt252);
    if let Option::Some(val) = x {
        println!("{val}");
    }
}
"#;

const MATCHING_WITH_SIMPLE_STRUCTS_FIELD: &str = r#"
fn do_thing() {}

fn main() {
    let x = Option::Some(2);
    if let Option::Some(2) = x {
        do_thing();
    }
}
"#;

const MATCHING_WITH_SIMPLE_STRUCTS_FIELD_ALLOWED: &str = r#"
fn do_thing() {}

fn main() {
    let x = Option::Some(2);
    #[allow(equatable_if_let)]
    if let Option::Some(2) = x {
        do_thing();
    }
}
"#;

#[test]
fn simple_equality_cases_ok_diagnostics() {
    test_lint_diagnostics!(SIMPLE_EQUALITY_CASES_OK, @r#"
    "#);
}

#[test]
fn simple_equality_cases_ok_fixer() {
    test_lint_fixer!(SIMPLE_EQUALITY_CASES_OK, @r#"
    fn main() {
        let x = Option::Some(1_felt252);
        if x == Option::Some(1_felt252) {
            println!("Value");
        }
    }
    "#);
}

#[test]
fn complex_equality_destructuring_if_let_diagnostics() {
    test_lint_diagnostics!(COMPLEX_EQUALITY_DESTRUCTURING_IF_LET, @r#"
    "#);
}

#[test]
fn complex_equality_destructuring_if_let_fixer() {
    test_lint_fixer!(COMPLEX_EQUALITY_DESTRUCTURING_IF_LET, @r#"
    fn main() {
        let x = Option::Some(1_felt252);
        let y = Option::Some(1_felt252);
        if let (Option::Some(a), Option::Some(b)) = (x, y) {
            let _avoid_collapsible = 1_u32;
            if a == b {
                println!("{a}");
            }
        }
    }
    "#);
}

#[test]
fn simple_value_pattern_matching_diagnostics() {
    test_lint_diagnostics!(SIMPLE_VALUE_PATTERN_MATCHING, @r"
    Plugin diagnostic: `if let` pattern used for equatable value. Consider using a simple comparison `==` instead
     --> lib.cairo:4:5
        if let 2 = a {}
        ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn simple_value_pattern_matching_fixer() {
    test_lint_fixer!(SIMPLE_VALUE_PATTERN_MATCHING, @r#"
    fn main() {
        let a = 2;
        if a == 2 {}
    }
    "#);
}

#[test]
fn simple_value_pattern_matching_with_comment_diagnostics() {
    test_lint_diagnostics!(SIMPLE_VALUE_PATTERN_MATCHING_WITH_COMMENT, @r"
    Plugin diagnostic: `if let` pattern used for equatable value. Consider using a simple comparison `==` instead
     --> lib.cairo:4:5-6:5
          if let 2 = a {
     _____^
    |       // Just a comment.
    |     }
    |_____^
    ");
}

#[test]
fn simple_value_pattern_matching_with_comment_fixer() {
    test_lint_fixer!(SIMPLE_VALUE_PATTERN_MATCHING_WITH_COMMENT, @r"
    fn main() {
        let a = 2;
        if a == 2 {
          // Just a comment.
        }
    }
    ");
}

#[test]
fn enum_unit_variant_pattern_matching_diagnostics() {
    test_lint_diagnostics!(ENUM_UNIT_VARIANT_PATTERN_MATCHING, @r"
    Plugin diagnostic: `if let` pattern used for equatable value. Consider using a simple comparison `==` instead
     --> lib.cairo:9:5
        if let Enum::UnitVariant = e {}
        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn enum_unit_variant_pattern_matching_fixer() {
    test_lint_fixer!(ENUM_UNIT_VARIANT_PATTERN_MATCHING, @r"
    enum Enum {
        UnitVariant,
    }

    fn main() {
        let e = Enum::UnitVariant;

        if e == Enum::UnitVariant {}
    }
    ");
}

#[test]
fn complex_equality_destructuring_diagnostics() {
    test_lint_diagnostics!(COMPLEX_EQUALITY_DESTRUCTURING, @r#"
    "#);
}

#[test]
fn complex_equality_destructuring_fixer() {
    test_lint_fixer!(COMPLEX_EQUALITY_DESTRUCTURING, @r#"
    fn main() {
        let x = Option::Some(1_felt252);
        if let Option::Some(val) = x {
            println!("{val}");
        }
    }
    "#);
}

#[test]
fn matching_with_simple_structs_field_diagnostics() {
    test_lint_diagnostics!(MATCHING_WITH_SIMPLE_STRUCTS_FIELD, @r"
    Plugin diagnostic: `if let` pattern used for equatable value. Consider using a simple comparison `==` instead
     --> lib.cairo:6:5-8:5
          if let Option::Some(2) = x {
     _____^
    |         do_thing();
    |     }
    |_____^
    ");
}

#[test]
fn matching_with_simple_structs_field_fixer() {
    test_lint_fixer!(MATCHING_WITH_SIMPLE_STRUCTS_FIELD, @r#"
    fn do_thing() {}

    fn main() {
        let x = Option::Some(2);
        if x == Option::Some(2) {
            do_thing();
        }
    }
    "#);
}

#[test]
fn matching_with_simple_structs_field_allowed_diagnostics() {
    test_lint_diagnostics!(MATCHING_WITH_SIMPLE_STRUCTS_FIELD_ALLOWED, @r#"
    "#);
}

#[test]
fn matching_with_simple_structs_field_allowed_fixer() {
    test_lint_fixer!(MATCHING_WITH_SIMPLE_STRUCTS_FIELD_ALLOWED, @r#"
    fn do_thing() {}

    fn main() {
        let x = Option::Some(2);
        #[allow(equatable_if_let)]
        if let Option::Some(2) = x {
            do_thing();
        }
    }
    "#);
}
