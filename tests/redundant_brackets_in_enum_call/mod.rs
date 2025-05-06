use crate::{test_lint_diagnostics, test_lint_fixer};

const REDUNDANT_BRACKET_CALL: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty
}
  
fn main() {
    let _a = MyEnum::Empty(()); 
}
"#;

const MULTIPLE_REDUNDANT_BRACKETS: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty1,
    Empty2, 
    Empty3
}
  
fn main() {
    let _a = MyEnum::Empty1(   ( ) ); // Comment
    let _b = MyEnum::Empty2((  ));
    let _c = MyEnum::Empty3;
}
"#;

const ALLOW_MULTIPLE_REDUNDANT_BRACKETS: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty1,
    Empty2,
    Empty3
}

#[allow(redundant_brackets_in_enum_call)]
fn main() {
    let _a = MyEnum::Empty1;
    let _b = MyEnum::Empty2(());
    let _c = MyEnum::Empty3;
}
"#;

const TUPLE_VARIANT: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Tuple: (u8, u8),
}

fn main() {
    let _a = MyEnum::Tuple((1,2));
}
"#;

const UNIT_SOME: &str = r#"
fn main() {
    let _a: Option<()> = Option::Some(());
}
"#;

const UNIT_OK: &str = r#"
fn main() {
    let _a: Result<(), ()> = Result::Ok(());
}
"#;

const UNIT_ERR: &str = r#"
fn main() {
    let _a: Result<(), ()> = Result::Err(());
}
"#;

const UNIT_GENERIC: &str = r#"
#[derive(Drop)]
enum MyEnum< T> {
    V:   T,
    Value: u8,
    Empty: ()
}

fn main() {
    let _a = MyEnum::V(());
    let _b = MyEnum::<()>::Value(1);
    let _c = MyEnum::<()>::Empty(());
}
"#;

#[test]
fn redundant_bracket_call_diagnostics() {
    test_lint_diagnostics!(REDUNDANT_BRACKET_CALL, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:9:14
        let _a = MyEnum::Empty(()); 
                 ^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_bracket_call_fixer() {
    test_lint_fixer!(REDUNDANT_BRACKET_CALL, @r"
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty
    }
      
    fn main() {
        let _a = MyEnum::Empty; 
    }
    ");
}

#[test]
fn multiple_empty_variants_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_REDUNDANT_BRACKETS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:11:14
        let _a = MyEnum::Empty1(   ( ) ); // Comment
                 ^^^^^^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:12:14
        let _b = MyEnum::Empty2((  ));
                 ^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn multiple_empty_variants_fixer() {
    test_lint_fixer!(MULTIPLE_REDUNDANT_BRACKETS, @r"
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty1,
        Empty2, 
        Empty3
    }
      
    fn main() {
        let _a = MyEnum::Empty1; // Comment
        let _b = MyEnum::Empty2;
        let _c = MyEnum::Empty3;
    }
    ");
}

#[test]
fn allow_multiple_empty_variants_diagnostics() {
    test_lint_diagnostics!(ALLOW_MULTIPLE_REDUNDANT_BRACKETS, @r"");
}

#[test]
fn allow_multiple_empty_variants_fixer() {
    test_lint_diagnostics!(ALLOW_MULTIPLE_REDUNDANT_BRACKETS, @r"");
}

#[test]
fn tuple_variant_diagnostics() {
    test_lint_diagnostics!(TUPLE_VARIANT, @r"");
}

#[test]
fn unit_some_diagnostic() {
    test_lint_diagnostics!(UNIT_SOME, @r"");
}

#[test]
fn unit_some_fixer() {
    test_lint_fixer!(UNIT_SOME, @r"
    fn main() {
        let _a: Option<()> = Option::Some(());
    }
    ");
}

#[test]
fn unit_ok_diagnostic() {
    test_lint_diagnostics!(UNIT_OK, @r"");
}

#[test]
fn unit_ok_fixer() {
    test_lint_fixer!(UNIT_OK, @r"
    fn main() {
        let _a: Result<(), ()> = Result::Ok(());
    }
    ");
}

#[test]
fn unit_err_diagnostic() {
    test_lint_diagnostics!(UNIT_ERR, @r"");
}

#[test]
fn unit_err_fixer() {
    test_lint_fixer!(UNIT_ERR, @r"
    fn main() {
        let _a: Result<(), ()> = Result::Err(());
    }
    ");
}

#[test]
fn unit_generic_diagnostics() {
    test_lint_diagnostics!(UNIT_GENERIC, @r"
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:6:5
        Empty: ()
        ^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:12:14
        let _c = MyEnum::<()>::Empty(());
                 ^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn unit_generic_fixer() {
    test_lint_fixer!(UNIT_GENERIC, @r"
    #[derive(Drop)]
    enum MyEnum< T> {
        V:   T,
        Value: u8,
        Empty
    }

    fn main() {
        let _a = MyEnum::V(());
        let _b = MyEnum::<()>::Value(1);
        let _c = MyEnum::<()>::Empty;
    }
    ");
}
