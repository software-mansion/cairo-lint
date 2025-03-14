use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_DOUBLE_PARENS: &str = r#"
fn main() -> u32 {
    ((0))
}
"#;

const SIMPLE_DOUBLE_PARENS_WITH_COMMENT: &str = r#"
fn main() -> u32 {
    ((
    // Just a comment.
    0
    ))
}
"#;

const UNNECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION: &str = r#"
fn main() -> u32 {
    ((3 + 5))
}
"#;

const NECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION: &str = r#"
fn main() -> u32 {
    2 * (3 + 5)
}
"#;

const TUPLE_DOUBLE_PARENS: &str = r#"
fn main() -> (felt252, felt252) {
    ((1, 2))
}
"#;

const ASSERT_EXPRESSIONS: &str = r#"
fn main() {
    assert!(((5)) == 4);
}
"#;

const DOUBLE_PARENS_WITH_FUNCTION_CALL: &str = r#"
fn foo(x: felt252) -> felt252 {
    x * 2
}

fn main() -> felt252 {
    ((foo(10)))
}
"#;

const DOUBLE_PARENS_WITH_RETURN: &str = r#"
fn main() -> felt252 {
    return ((5 + 7));
}
"#;

const DOUBLE_PARENS_IN_LET_STATEMENT: &str = r#"
fn main() {
    let _x = ((10 * 2));
}
"#;

const DOUBLE_PARENS_IN_LET_STATEMENT_ALLOWED: &str = r#"
fn main() {
    #[allow(double_parens)]
    let _x = ((10 * 2));
}
"#;

const DOUBLE_PARENS_IN_STRUCT_FIELD_ACCESS: &str = r#"
struct MyStruct {
    x: felt252,
    y: felt252,
}

fn main() -> felt252 {
    let my_struct = MyStruct { x: 10, y: 20 };
    return ((my_struct.y));
}
"#;

const DOUBLE_PARENS_IN_MATCH_ARM: &str = r#"
fn main() -> felt252 {
    let x = 5;
    match x {
        1 => ((10)),
        5 => ((20)),
        _ => ((30)),
    }
}
"#;

#[test]
fn simple_double_parens_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DOUBLE_PARENS, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
      |
    3 |     ((0))
      |     -----
      |
    ");
}

#[test]
fn simple_double_parens_fixer() {
    test_lint_fixer!(SIMPLE_DOUBLE_PARENS, @r"
    fn main() -> u32 {
        0}
    ");
}

#[test]
fn simple_double_parens_with_comment_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DOUBLE_PARENS_WITH_COMMENT, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
      |
    3 | /     ((
    4 | |     // Just a comment.
    5 | |     0
    6 | |     ))
      | |______-
      |
    ");
}

#[test]
fn simple_double_parens_with_comment_fixer() {
    test_lint_fixer!(SIMPLE_DOUBLE_PARENS_WITH_COMMENT, @r"
    fn main() -> u32 {
        // Just a comment.
        0
    }
    ");
}

#[test]
fn unnecessary_parentheses_in_arithmetic_expression_diagnostics() {
    test_lint_diagnostics!(UNNECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
      |
    3 |     ((3 + 5))
      |     ---------
      |
    ");
}

#[test]
fn unnecessary_parentheses_in_arithmetic_expression_fixer() {
    test_lint_fixer!(UNNECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION, @r"
    fn main() -> u32 {
        3 + 5}
    ");
}

#[test]
fn necessary_parentheses_in_arithmetic_expression_diagnostics() {
    test_lint_diagnostics!(NECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION, @r#"
    "#);
}

#[test]
fn necessary_parentheses_in_arithmetic_expression_fixer() {
    test_lint_fixer!(NECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION, @r#"
    fn main() -> u32 {
        2 * (3 + 5)
    }
    "#);
}

#[test]
fn tuple_double_parens_diagnostics() {
    test_lint_diagnostics!(TUPLE_DOUBLE_PARENS, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
      |
    3 |     ((1, 2))
      |     --------
      |
    ");
}

#[test]
fn tuple_double_parens_fixer() {
    test_lint_fixer!(TUPLE_DOUBLE_PARENS, @r"
    fn main() -> (felt252, felt252) {
        (1, 2)}
    ");
}

#[test]
fn assert_expressions_diagnostics() {
    test_lint_diagnostics!(ASSERT_EXPRESSIONS, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:13
      |
    3 |     assert!(((5)) == 4);
      |             -----
      |
    ");
}

#[test]
fn assert_expressions_fixer() {
    test_lint_fixer!(ASSERT_EXPRESSIONS, @r"
    fn main() {
        assert!(5== 4);
    }
    ");
}

#[test]
fn double_parens_with_function_call_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_FUNCTION_CALL, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:7:5
      |
    7 |     ((foo(10)))
      |     -----------
      |
    ");
}

#[test]
fn double_parens_with_function_call_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_FUNCTION_CALL, @r"
    fn foo(x: felt252) -> felt252 {
        x * 2
    }

    fn main() -> felt252 {
        foo(10)}
    ");
}

#[test]
fn double_parens_with_return_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_RETURN, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:12
      |
    3 |     return ((5 + 7));
      |            ---------
      |
    ");
}

#[test]
fn double_parens_with_return_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_RETURN, @r#"
    fn main() -> felt252 {
        return 5 + 7;
    }
    "#);
}

#[test]
fn double_parens_in_let_statement_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_IN_LET_STATEMENT, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:14
      |
    3 |     let _x = ((10 * 2));
      |              ----------
      |
    ");
}

#[test]
fn double_parens_in_let_statement_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_IN_LET_STATEMENT, @r#"
    fn main() {
        let _x = 10 * 2;
    }
    "#);
}

#[test]
fn double_parens_in_let_statement_allowed_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_IN_LET_STATEMENT_ALLOWED, @r#"
    "#);
}

#[test]
fn double_parens_in_let_statement_allowed_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_IN_LET_STATEMENT_ALLOWED, @r#"
    fn main() {
        #[allow(double_parens)]
        let _x = ((10 * 2));
    }
    "#);
}

#[test]
fn double_parens_in_struct_field_access_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_IN_STRUCT_FIELD_ACCESS, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:9:12
      |
    9 |     return ((my_struct.y));
      |            ---------------
      |
    ");
}

#[test]
fn double_parens_in_struct_field_access_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_IN_STRUCT_FIELD_ACCESS, @r#"
    struct MyStruct {
        x: felt252,
        y: felt252,
    }

    fn main() -> felt252 {
        let my_struct = MyStruct { x: 10, y: 20 };
        return my_struct.y;
    }
    "#);
}

#[test]
fn double_parens_in_match_arm_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_IN_MATCH_ARM, @r"
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:5:14
      |
    5 |         1 => ((10)),
      |              ------
      |
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:14
      |
    6 |         5 => ((20)),
      |              ------
      |
    warning: Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:7:14
      |
    7 |         _ => ((30)),
      |              ------
      |
    ");
}

#[test]
fn double_parens_in_match_arm_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_IN_MATCH_ARM, @r#"
    fn main() -> felt252 {
        let x = 5;
        match x {
            1 => 10,
            5 => 20,
            _ => 30,
        }
    }
    "#);
}
