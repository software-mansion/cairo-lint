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

// TODO: https://github.com/software-mansion/cairo-lint/issues/327
// const ASSERT_EXPRESSIONS: &str = r#"
// fn main() {
//     assert!(((5)) == 4);
// }
// "#;

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

const DOUBLE_PARENS_WITH_NEGATION: &str = r#"
fn main() {
    let x = 5_u8;
    let y = 10;
    let _z = !((x < y));
}
"#;

const DOUBLE_PARENS_WITH_AND: &str = r#"
fn main() {
    let x = 5_u8;
    let y = 10;
    let compare = false;
    let _z = !(((compare))) && ((x < y));
}
"#;

const DOUBLE_PARENS_WITH_OR_SINGLE_VALUE: &str = r#"
fn main() {
    let x = 5_u8;
    let y = 10;
    let compare = false;
    let _z = ((((false)) || (x < y))) == compare;
}
"#;

const DOUBLE_PARENS_WITH_ARITHMETIC_EXPRESSION: &str = r#"
fn main() {
    let x = 5_u8;
    let y = 10;
    let _z = ((x + y)) * 2;
}
"#;

const DOUBLE_PARENS_WITH_INDEXED: &str = r#"
fn fun(c: Array<u8>) -> Array<u8> {
    let mut a = c;
    a.append(1);
    a
}

fn main() {
    let b = array![2,3];

    let _c = *((((fun(b)))[1])) + 2;
}
"#;

const DOUBLE_PARENS_NOT_FIRING_FOR_BINARY_EXPR_IN_FUNC_ARG: &str = r#"
fn func(number: felt252) {}

fn main() {
    let a: u128 = 5;
    let a_ref = @a;
    func((*a_ref).into());
}
"#;

const DOUBLE_PARENS_NOT_FIRING_FOR_NECESSARY_CASES: &str = r#"
fn main() {
    let a: u128 = 5;
    let a_ref = @a;
    let a_array: Array<@u128> = array![a_ref];
    let _unused_var: Array<felt252> = a_array
        .into_iter()
        .map(|alpha| -> felt252 {
            (*alpha).into()
        })
        .collect();
}

"#;

#[test]
fn simple_double_parens_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DOUBLE_PARENS, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
        ((0))
        ^^^^^
    ");
}

#[test]
fn simple_double_parens_fixer() {
    test_lint_fixer!(SIMPLE_DOUBLE_PARENS, @r"
    fn main() -> u32 {
        0
    }
    ");
}

#[test]
fn simple_double_parens_with_comment_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DOUBLE_PARENS_WITH_COMMENT, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5-6:6
          ((
     _____^
    | ...
    |     ))
    |______^
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
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
        ((3 + 5))
        ^^^^^^^^^
    ");
}

#[test]
fn unnecessary_parentheses_in_arithmetic_expression_fixer() {
    test_lint_fixer!(UNNECESSARY_PARENTHESES_IN_ARITHMETIC_EXPRESSION, @r"
    fn main() -> u32 {
        3 + 5
    }
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
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:5
        ((1, 2))
        ^^^^^^^^
    ");
}

#[test]
fn tuple_double_parens_fixer() {
    test_lint_fixer!(TUPLE_DOUBLE_PARENS, @r"
    fn main() -> (felt252, felt252) {
        (1, 2)
    }
    ");
}

// #[test]
// fn assert_expressions_diagnostics() {
//     test_lint_diagnostics!(ASSERT_EXPRESSIONS, @r"
//     Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
//      --> lib.cairo:3:13
//         assert!(((5)) == 4);
//                 ^^^^^
//     ");
// }

// #[test]
// fn assert_expressions_fixer() {
//     test_lint_fixer!(ASSERT_EXPRESSIONS, @r"
//     fn main() {
//         assert!(5== 4);
//     }
//     ");
// }

#[test]
fn double_parens_with_function_call_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_FUNCTION_CALL, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:7:5
        ((foo(10)))
        ^^^^^^^^^^^
    ");
}

#[test]
fn double_parens_with_function_call_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_FUNCTION_CALL, @r"
    fn foo(x: felt252) -> felt252 {
        x * 2
    }

    fn main() -> felt252 {
        foo(10)
    }
    ");
}

#[test]
fn double_parens_with_return_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_RETURN, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:12
        return ((5 + 7));
               ^^^^^^^^^
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
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:3:14
        let _x = ((10 * 2));
                 ^^^^^^^^^^
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
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:9:12
        return ((my_struct.y));
               ^^^^^^^^^^^^^^^
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
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:5:14
            1 => ((10)),
                 ^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:14
            5 => ((20)),
                 ^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:7:14
            _ => ((30)),
                 ^^^^^^
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

#[test]
fn double_parens_with_negation_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_NEGATION, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:5:15
        let _z = !((x < y));
                  ^^^^^^^^^
    ")
}

#[test]
fn double_parens_with_negation_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_NEGATION, @r"
    fn main() {
        let x = 5_u8;
        let y = 10;
        let _z = !(x < y);
    }
    ")
}

#[test]
fn double_parens_with_and_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_AND, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:15
        let _z = !(((compare))) && ((x < y));
                  ^^^^^^^^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:16
        let _z = !(((compare))) && ((x < y));
                   ^^^^^^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:32
        let _z = !(((compare))) && ((x < y));
                                   ^^^^^^^^^
    ")
}

#[test]
fn double_parens_with_and_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_AND, @r"
    fn main() {
        let x = 5_u8;
        let y = 10;
        let compare = false;
        let _z = !compare && (x < y);
    }
    ")
}

#[test]
fn double_parens_with_or_single_value_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_OR_SINGLE_VALUE, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:14
        let _z = ((((false)) || (x < y))) == compare;
                 ^^^^^^^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:6:16
        let _z = ((((false)) || (x < y))) == compare;
                   ^^^^^^^^^
    ")
}

#[test]
fn double_parens_with_or_single_value_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_OR_SINGLE_VALUE, @r"
    fn main() {
        let x = 5_u8;
        let y = 10;
        let compare = false;
        let _z = (false || (x < y)) == compare;
    }
    ")
}

#[test]
fn double_parens_with_arithmetic_expression_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_ARITHMETIC_EXPRESSION, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:5:14
        let _z = ((x + y)) * 2;
                 ^^^^^^^^^
    ")
}

#[test]
fn double_parens_with_arithmetic_expression_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_ARITHMETIC_EXPRESSION, @r"
    fn main() {
        let x = 5_u8;
        let y = 10;
        let _z = (x + y) * 2;
    }
    ")
}

#[test]
fn double_parens_with_indexed_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_WITH_INDEXED, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:11:15
        let _c = *((((fun(b)))[1])) + 2;
                  ^^^^^^^^^^^^^^^^^
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:11:17
        let _c = *((((fun(b)))[1])) + 2;
                    ^^^^^^^^^^
    ")
}

#[test]
fn double_parens_with_indexed_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_WITH_INDEXED, @r"
    fn fun(c: Array<u8>) -> Array<u8> {
        let mut a = c;
        a.append(1);
        a
    }

    fn main() {
        let b = array![2, 3];

        let _c = *fun(b)[1] + 2;
    }
    ")
}

#[test]
fn double_parens_in_single_arg_function_call_diagnostics() {
    test_lint_diagnostics!(r#"
    fn func(c: u8) {}

    fn main() {
        func((5));
    }
    "#, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:4:10
        func((5));
             ^^^
    ")
}

#[test]
fn double_parens_in_single_arg_function_call_fixer() {
    test_lint_fixer!(r#"
    fn func(c: u8) {}

    fn main() {
        func((5));
    }
    "#, @r"
    fn func(c: u8) {}

    fn main() {
        func(5);
    }
    ")
}

#[test]
fn triple_parens_in_single_arg_tuple_function_call_diagnostics() {
    test_lint_diagnostics!(r#"
    fn func(a: (u8, felt252)) {}

    fn main() {
        func(((5, 6)));
    }
    "#, @r"
    Plugin diagnostic: unnecessary double parentheses found. Consider removing them.
     --> lib.cairo:4:10
        func(((5, 6)));
             ^^^^^^^^
    ")
}

#[test]
fn triple_parens_in_single_arg_tuple_function_call_fixer() {
    test_lint_fixer!(r#"
    fn func(a: (u8, felt252)) {}

    fn main() {
        func(((5, 6)));
    }
    "#, @r"
    fn func(a: (u8, felt252)) {}

    fn main() {
        func((5, 6));
    }
    ")
}

#[test]
fn double_parens_in_single_arg_function_call_unit_type() {
    test_lint_diagnostics!(r#"
    fn func(c: ()) {}

    fn main() {
        func(());
    }
    "#, @"")
}

#[test]
fn double_parens_in_function_call_around_multiple_args() {
    test_lint_diagnostics!(r#"
    fn func(a: u8, b: felt252) {}

    fn main() {
        func((5, 6));
    }
    "#, @r"
    Wrong number of arguments. Expected 2, found: 1
     --> lib.cairo:4:5
        func((5, 6));
        ^^^^^^^^^^^^
    ")
}

#[test]
fn double_parens_in_function_call_when_expecting_tuple() {
    test_lint_diagnostics!(r#"
    fn func(a: (u8, felt252)) {}

    fn main() {
        func((5, 6));
    }
    "#, @"")
}

#[test]
fn double_parens_in_function_call_with_multiple_args_around_single_arg() {
    test_lint_diagnostics!(r#"
    fn func(a: (u8, felt252)) {}

    fn main() {
        func((5), 6);
    }
    "#, @r"
    Wrong number of arguments. Expected 1, found: 2
     --> lib.cairo:4:5
        func((5), 6);
        ^^^^^^^^^^^^
    ")
}

#[test]
fn double_parens_not_firing_for_binary_expr_in_func_arg_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_NOT_FIRING_FOR_BINARY_EXPR_IN_FUNC_ARG, @"");
}

#[test]
fn double_parens_not_firing_for_binary_expr_in_func_arg_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_NOT_FIRING_FOR_BINARY_EXPR_IN_FUNC_ARG, @r#"
    fn func(number: felt252) {}

    fn main() {
        let a: u128 = 5;
        let a_ref = @a;
        func((*a_ref).into());
    }
    "#);
}

#[test]
fn double_parens_not_firing_for_necessary_cases_diagnostics() {
    test_lint_diagnostics!(DOUBLE_PARENS_NOT_FIRING_FOR_NECESSARY_CASES, @"");
}

#[test]
fn double_parens_not_firing_for_necessary_cases_fixer() {
    test_lint_fixer!(DOUBLE_PARENS_NOT_FIRING_FOR_NECESSARY_CASES, @r#"
    fn main() {
        let a: u128 = 5;
        let a_ref = @a;
        let a_array: Array<@u128> = array![a_ref];
        let _unused_var: Array<felt252> = a_array
            .into_iter()
            .map(|alpha| -> felt252 {
                (*alpha).into()
            })
            .collect();
    }
    "#);
}
