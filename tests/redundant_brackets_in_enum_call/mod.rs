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
