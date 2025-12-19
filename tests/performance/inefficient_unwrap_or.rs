use crate::{test_lint_diagnostics, test_lint_fixer};

const OPTION_FUNCTION_CALL_NONTRIVIAL: &str = r#"
fn bar() -> usize {
    0
}

fn foo() {
    let x = Option::<usize>::None;
    let _ = x.unwrap_or(bar());
}
"#;

const OPTION_MATCH_EXPRESSION_NONTRIVIAL: &str = r#"
enum Enum {
    First,
    Second,
}

fn foo() {
    let x = Option::<usize>::None;
    let y = Enum::First;
    let _ = x.unwrap_or(match y {
        Enum::First => 1,
        Enum::Second => 2,
    });
}
"#;

const OPTION_IF_EXPRESSION_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<usize>::None;
    let y: usize = 0;
    let _ = x.unwrap_or(if y > 1 { 0 } else { 1 });
}
"#;

const OPTION_BLOCK_EXPRESSION_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<usize>::None;
    let _ = x.unwrap_or({
        let a = 1;
        let b = 2;
        a + b
    });
}
"#;

const OPTION_LOOP_EXPRESSION_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<usize>::None;
    let _ = x.unwrap_or(loop {
        break 0;
    });
}
"#;

const OPTION_WHILE_LOOP_EXPRESSION_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<()>::None;
    let mut y: usize = 0;
    let _ = x.unwrap_or(while y != 5 {
        y += 1;
        break;
    });
}
"#;

const OPTION_FOR_EXPRESSION_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<()>::None;
    let y = array![0x0, 0x1, 0x3];
    let _ = x.unwrap_or(for _ in y {});
}
"#;

const OPTION_ARRAY_NONTRIVIAL: &str = r#"
fn foo() {
    let x = Option::<Array<felt252>>::None;
    let _ = x.unwrap_or(array![0, 1, 2]);
}
"#;

const OPTION_TUPLE_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::<(usize, usize)>::None;
    let _ = x.unwrap_or((0, 0));
}
"#;

const OPTION_TUPLE_NONTRIVIAL: &str = r#"
fn bar() -> usize {
    0
}

fn foo() {
    let x = Option::<(usize, usize)>::None;
    let _ = x.unwrap_or((0, bar()));
}
"#;

const OPTION_SNAPSHOT_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::<@usize>::None;
    let _ = x.unwrap_or(@0);
}
"#;

const OPTION_SNAPSHOT_NONTRIVIAL: &str = r#"
fn bar() -> usize {
    0
}

fn foo() {
    let x = Option::<@usize>::None;
    let _ = x.unwrap_or(@bar());
}
"#;

const OPTION_DESNAP_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::<usize>::None;
    let y: @usize = @0;
    let _ = x.unwrap_or(*y);
}
"#;

const OPTION_DESNAP_NONTRIVIAL: &str = r#"
fn bar() -> @usize {
    @0
}

fn foo() {
    let x = Option::<usize>::None;
    let _ = x.unwrap_or(*bar());
}
"#;

const OPTION_STRUCT_CONSTRUCTOR_TRIVIAL: &str = r#"
#[derive(Destruct)]
struct Struct {
    x: felt252
}

fn foo() {
    let x = Option::<Struct>::None;
    let _ = x.unwrap_or(Struct { x: 0x0 });
}
"#;

const OPTION_STRUCT_CONSTRUCTOR_NONTRIVIAL: &str = r#"
#[derive(Destruct)]
struct Struct {
    x: felt252
}

fn bar() -> felt252 {
    0
}

fn foo() {
    let x = Option::<Struct>::None;
    let _ = x.unwrap_or(Struct { x: bar() });
}
"#;

const OPTION_ENUM_CONSTRUCTOR_TRIVIAL: &str = r#"
#[derive(Destruct)]
enum Enum {
    X: felt252,
    Y,
}

fn foo() {
    let x = Option::<Enum>::None;
    let _ = x.unwrap_or(Enum::X(0));
}
"#;

const OPTION_ENUM_CONSTRUCTOR_NONTRIVIAL: &str = r#"
#[derive(Destruct)]
enum Enum {
    X: felt252,
    Y,
}

fn bar() -> felt252 {
    0
}

fn foo() {
    let x = Option::<Enum>::None;
    let _ = x.unwrap_or(Enum::X(bar()));
}
"#;

const OPTION_LOGICAL_OPERATOR_TRIVIAL: &str = r#"
fn foo(x: usize) {
    let x = Option::<bool>::None;
    let _ = x.unwrap_or(true && false);
}
"#;

const OPTION_LOGICAL_OPERATOR_NONTRIVIAL: &str = r#"
fn bar(x: usize) -> bool {
    x > 1
}

fn foo(x: usize) {
    let _ = Option::<bool>::None.unwrap_or(bar(x) && false);
}
"#;

const OPTION_MEMBER_ACCESS_TRIVAL: &str = r#"
#[derive(Destruct)]
struct Struct {
    x: felt252
}

fn foo() {
    let s = Struct { x: 0x0 };
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or(s.x);
}
"#;

const OPTION_MEMBER_ACCESS_NONTRIVIAL: &str = r#"
#[derive(Destruct)]
struct Struct {
    x: felt252
}

struct Struct2 {
    y: felt252
}

impl ImplDeref of Deref<Struct> {
    type Target = Struct2;
    fn deref(self: Struct) -> Struct2 {
        Struct2 { y: self.x }
    }
}

fn foo() {
    let s = Struct { x: 0x0 };
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or(s.x);
    let _ = x.unwrap_or(s.y);
}
"#;

const OPTION_CONSTANT_TRIVIAL: &str = r#"
const CONST: felt252 = 0;

fn foo() {
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or(CONST);
}
"#;

const OPTION_VAR_TRIVIAL: &str = r#"
fn foo() {
    let x = 0;
    let y = Option::<felt252>::None;
    let _ = y.unwrap_or(x);
}
"#;

const OPTION_LITERAL_NUMERIC_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or(0);
}
"#;

const OPTION_STRING_LITERAL_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or('text');
}
"#;

const OPTION_CLOSURE_TRIVIAL: &str = r#"
fn foo() {
    let x = Option::None;
    let _ = x.unwrap_or(|x| x + 1);
}
"#;

const OPTION_PROPAGATE_ERROR_TRIVIAL: &str = r#"
fn foo() -> Result<felt252, felt252> {
    let err = Result::Err(0);
    let x = Option::<felt252>::None;
    let _ = x.unwrap_or(err?);
    Ok(0)
}
"#;

#[test]
fn test_option_function_call_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_FUNCTION_CALL_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:8:13
        let _ = x.unwrap_or(bar());
                ^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_function_call_nontrivial_fixer() {
    test_lint_fixer!(OPTION_FUNCTION_CALL_NONTRIVIAL, @r"
    fn bar() -> usize {
        0
    }

    fn foo() {
        let x = Option::<usize>::None;
        let _ = x.unwrap_or_else(|| bar());
    }
    ")
}

#[test]
fn test_option_match_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_MATCH_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:10:13-13:6
          let _ = x.unwrap_or(match y {
     _____________^
    | ...
    |     });
    |______^
    ")
}

#[test]
fn test_option_match_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_MATCH_EXPRESSION_NONTRIVIAL, @r"
    enum Enum {
        First,
        Second,
    }

    fn foo() {
        let x = Option::<usize>::None;
        let y = Enum::First;
        let _ = x.unwrap_or_else(|| match y {
            Enum::First => 1,
            Enum::Second => 2,
        });
    }
    ")
}

#[test]
fn test_option_if_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_IF_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:5:13
        let _ = x.unwrap_or(if y > 1 { 0 } else { 1 });
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_if_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_IF_EXPRESSION_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<usize>::None;
        let y: usize = 0;
        let _ = x.unwrap_or_else(|| if y > 1 {
            0
        } else {
            1
        });
    }
    ")
}

#[test]
fn test_option_block_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_BLOCK_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:4:13-8:6
          let _ = x.unwrap_or({
     _____________^
    | ...
    |     });
    |______^
    ")
}

#[test]
fn test_option_block_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_BLOCK_EXPRESSION_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<usize>::None;
        let _ = x.unwrap_or_else(|| {
            let a = 1;
            let b = 2;
            a + b
        });
    }
    ")
}

#[test]
fn test_option_loop_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_LOOP_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:4:13-6:6
          let _ = x.unwrap_or(loop {
     _____________^
    |         break 0;
    |     });
    |______^
    ")
}

#[test]
fn test_option_loop_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_LOOP_EXPRESSION_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<usize>::None;
        let _ = x.unwrap_or_else(|| loop {
            break 0;
        });
    }
    ")
}

#[test]
fn test_option_while_loop_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_WHILE_LOOP_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:5:13-8:6
          let _ = x.unwrap_or(while y != 5 {
     _____________^
    | ...
    |     });
    |______^
    ")
}

#[test]
fn test_option_while_loop_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_WHILE_LOOP_EXPRESSION_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<()>::None;
        let mut y: usize = 0;
        let _ = x.unwrap_or_else(|| while y != 5 {
            y += 1;
            break;
        });
    }
    ")
}

#[test]
fn test_option_for_expression_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_FOR_EXPRESSION_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:5:13
        let _ = x.unwrap_or(for _ in y {});
                ^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_for_expression_nontrivial_fixer() {
    test_lint_fixer!(OPTION_FOR_EXPRESSION_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<()>::None;
        let y = array![0x0, 0x1, 0x3];
        let _ = x.unwrap_or_else(|| for _ in y {});
    }
    ")
}

#[test]
fn test_option_array_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_ARRAY_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:4:13
        let _ = x.unwrap_or(array![0, 1, 2]);
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_array_nontrivial_fixer() {
    test_lint_fixer!(OPTION_ARRAY_NONTRIVIAL, @r"
    fn foo() {
        let x = Option::<Array<felt252>>::None;
        let _ = x.unwrap_or_else(|| array![0, 1, 2]);
    }
    ")
}

#[test]
fn test_option_tuple_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_TUPLE_TRIVIAL, @"")
}

#[test]
fn test_option_tuple_trivial_fixer() {
    test_lint_fixer!(OPTION_TUPLE_TRIVIAL, @r"
    fn foo() {
        let x = Option::<(usize, usize)>::None;
        let _ = x.unwrap_or((0, 0));
    }
    ")
}

#[test]
fn test_option_tuple_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_TUPLE_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:8:13
        let _ = x.unwrap_or((0, bar()));
                ^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_tuple_nontrivial_fixer() {
    test_lint_fixer!(OPTION_TUPLE_NONTRIVIAL, @r"
    fn bar() -> usize {
        0
    }

    fn foo() {
        let x = Option::<(usize, usize)>::None;
        let _ = x.unwrap_or_else(|| (0, bar()));
    }
    ")
}

#[test]
fn test_option_snapshot_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_SNAPSHOT_TRIVIAL, @"")
}

#[test]
fn test_option_snapshot_trivial_fixer() {
    test_lint_fixer!(OPTION_SNAPSHOT_TRIVIAL, @r"
    fn foo() {
        let x = Option::<@usize>::None;
        let _ = x.unwrap_or(@0);
    }
    ")
}

#[test]
fn test_option_snapshot_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_SNAPSHOT_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:8:13
        let _ = x.unwrap_or(@bar());
                ^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_snapshot_nontrivial_fixer() {
    test_lint_fixer!(OPTION_SNAPSHOT_NONTRIVIAL, @r"
    fn bar() -> usize {
        0
    }

    fn foo() {
        let x = Option::<@usize>::None;
        let _ = x.unwrap_or_else(|| @bar());
    }
    ")
}

#[test]
fn test_option_desnap_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_DESNAP_TRIVIAL, @"")
}

#[test]
fn test_option_desnap_trivial_fixer() {
    test_lint_fixer!(OPTION_DESNAP_TRIVIAL, @r"
    fn foo() {
        let x = Option::<usize>::None;
        let y: @usize = @0;
        let _ = x.unwrap_or(*y);
    }
    ")
}

#[test]
fn test_option_desnap_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_DESNAP_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:8:13
        let _ = x.unwrap_or(*bar());
                ^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_desnap_nontrivial_fixer() {
    test_lint_fixer!(OPTION_DESNAP_NONTRIVIAL, @r"
    fn bar() -> @usize {
        @0
    }

    fn foo() {
        let x = Option::<usize>::None;
        let _ = x.unwrap_or_else(|| *bar());
    }
    ")
}

#[test]
fn test_option_struct_constructor_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_STRUCT_CONSTRUCTOR_TRIVIAL, @"")
}

#[test]
fn test_option_struct_constructor_trivial_fixer() {
    test_lint_fixer!(OPTION_STRUCT_CONSTRUCTOR_TRIVIAL, @r"
    #[derive(Destruct)]
    struct Struct {
        x: felt252,
    }

    fn foo() {
        let x = Option::<Struct>::None;
        let _ = x.unwrap_or(Struct { x: 0x0 });
    }
    ")
}

#[test]
fn test_option_struct_constructor_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_STRUCT_CONSTRUCTOR_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:13:13
        let _ = x.unwrap_or(Struct { x: bar() });
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_struct_constructor_nontrivial_fixer() {
    test_lint_fixer!(OPTION_STRUCT_CONSTRUCTOR_NONTRIVIAL, @r"
    #[derive(Destruct)]
    struct Struct {
        x: felt252,
    }

    fn bar() -> felt252 {
        0
    }

    fn foo() {
        let x = Option::<Struct>::None;
        let _ = x.unwrap_or_else(|| Struct { x: bar() });
    }
    ")
}

#[test]
fn test_option_enum_constructor_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_ENUM_CONSTRUCTOR_TRIVIAL, @"")
}

#[test]
fn test_option_enum_constructor_trivial_fixer() {
    test_lint_fixer!(OPTION_ENUM_CONSTRUCTOR_TRIVIAL, @r"
    #[derive(Destruct)]
    enum Enum {
        X: felt252,
        Y,
    }

    fn foo() {
        let x = Option::<Enum>::None;
        let _ = x.unwrap_or(Enum::X(0));
    }
    ")
}

#[test]
fn test_option_enum_constructor_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_ENUM_CONSTRUCTOR_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:14:13
        let _ = x.unwrap_or(Enum::X(bar()));
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_enum_constructor_nontrivial_fixer() {
    test_lint_fixer!(OPTION_ENUM_CONSTRUCTOR_NONTRIVIAL, @r"
    #[derive(Destruct)]
    enum Enum {
        X: felt252,
        Y,
    }

    fn bar() -> felt252 {
        0
    }

    fn foo() {
        let x = Option::<Enum>::None;
        let _ = x.unwrap_or_else(|| Enum::X(bar()));
    }
    ")
}

#[test]
fn test_option_logical_operator_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_LOGICAL_OPERATOR_TRIVIAL, @"")
}

#[test]
fn test_option_logical_operator_trivial_fixer() {
    test_lint_fixer!(OPTION_LOGICAL_OPERATOR_TRIVIAL, @r"
    fn foo(x: usize) {
        let x = Option::<bool>::None;
        let _ = x.unwrap_or(true && false);
    }
    ")
}

#[test]
fn test_option_logical_operator_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_LOGICAL_OPERATOR_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:7:13
        let _ = Option::<bool>::None.unwrap_or(bar(x) && false);
                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_logical_operator_nontrivial_fixer() {
    test_lint_fixer!(OPTION_LOGICAL_OPERATOR_NONTRIVIAL, @r"
    fn bar(x: usize) -> bool {
        x > 1
    }

    fn foo(x: usize) {
        let _ = Option::<bool>::None.unwrap_or_else(|| bar(x) && false);
    }
    ")
}

#[test]
fn test_option_member_access_trival_diagnostics() {
    test_lint_diagnostics!(OPTION_MEMBER_ACCESS_TRIVAL, @"")
}

#[test]
fn test_option_member_access_trival_fixer() {
    test_lint_fixer!(OPTION_MEMBER_ACCESS_TRIVAL, @r"
    #[derive(Destruct)]
    struct Struct {
        x: felt252,
    }

    fn foo() {
        let s = Struct { x: 0x0 };
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or(s.x);
    }
    ")
}

#[test]
fn test_option_member_access_nontrivial_diagnostics() {
    test_lint_diagnostics!(OPTION_MEMBER_ACCESS_NONTRIVIAL, @r"
    Plugin diagnostic: Inefficient `unwrap_or` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:22:13
        let _ = x.unwrap_or(s.y);
                ^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn test_option_member_access_nontrivial_fixer() {
    test_lint_fixer!(OPTION_MEMBER_ACCESS_NONTRIVIAL, @r"
    #[derive(Destruct)]
    struct Struct {
        x: felt252,
    }

    struct Struct2 {
        y: felt252,
    }

    impl ImplDeref of Deref<Struct> {
        type Target = Struct2;
        fn deref(self: Struct) -> Struct2 {
            Struct2 { y: self.x }
        }
    }

    fn foo() {
        let s = Struct { x: 0x0 };
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or(s.x);
        let _ = x.unwrap_or_else(|| s.y);
    }
    ")
}

#[test]
fn test_option_constant_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_CONSTANT_TRIVIAL, @"")
}

#[test]
fn test_option_constant_trivial_fixer() {
    test_lint_fixer!(OPTION_CONSTANT_TRIVIAL, @r"
    const CONST: felt252 = 0;

    fn foo() {
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or(CONST);
    }
    ")
}

#[test]
fn test_option_var_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_VAR_TRIVIAL, @"")
}

#[test]
fn test_option_var_trivial_fixer() {
    test_lint_fixer!(OPTION_VAR_TRIVIAL, @r"
    fn foo() {
        let x = 0;
        let y = Option::<felt252>::None;
        let _ = y.unwrap_or(x);
    }
    ")
}

#[test]
fn test_option_literal_numeric_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_LITERAL_NUMERIC_TRIVIAL, @"")
}

#[test]
fn test_option_literal_numeric_trivial_fixer() {
    test_lint_fixer!(OPTION_LITERAL_NUMERIC_TRIVIAL, @r"
    fn foo() {
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or(0);
    }
    ")
}

#[test]
fn test_option_string_literal_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_STRING_LITERAL_TRIVIAL, @"")
}

#[test]
fn test_option_string_literal_trivial_fixer() {
    test_lint_fixer!(OPTION_STRING_LITERAL_TRIVIAL, @r"
    fn foo() {
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or('text');
    }
    ")
}

#[test]
fn test_option_closure_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_CLOSURE_TRIVIAL, @"")
}

#[test]
fn test_option_closure_trivial_fixer() {
    test_lint_fixer!(OPTION_CLOSURE_TRIVIAL, @r"
    fn foo() {
        let x = Option::None;
        let _ = x.unwrap_or(|x| x + 1);
    }
    ")
}

#[test]
fn test_option_propagate_error_trivial_diagnostics() {
    test_lint_diagnostics!(OPTION_PROPAGATE_ERROR_TRIVIAL, @"")
}

#[test]
fn test_option_propagate_error_trivial_fixer() {
    test_lint_fixer!(OPTION_PROPAGATE_ERROR_TRIVIAL, @r"
    fn foo() -> Result<felt252, felt252> {
        let err = Result::Err(0);
        let x = Option::<felt252>::None;
        let _ = x.unwrap_or(err?);
        Ok(0)
    }
    ")
}
