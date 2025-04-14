use crate::{test_lint_diagnostics, test_lint_fixer};

const MULTIPLE_EMPTY_VARIANTS: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty1: (), // Some comment
    Empty2: (        ),         // Different comment
    Empty3
}
  
fn main() {
    let _a = MyEnum::Empty1(   ( ) );
    let _b = MyEnum::Empty2((  ));
    let _c = MyEnum::Empty3;
}
"#;

const CORRECT_VARIANT: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty,
}
  
fn main() {
    let _a = MyEnum::Empty; 
}
"#;

const MISMATCHED_BRACKET_VARIANT: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty: () // Comment
}
  
fn main() {
    let _a = MyEnum::Empty;
}
"#;

const EXTRA_BRACKETS_VARIANT: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty
}
  
fn main() {
    let _a = MyEnum::Empty(()); 
}
"#;

const ALLOW_MULTIPLE_EMPTY_VARIANTS: &str = r#"
#[derive(Drop)]
#[allow(empty_enum_brackets_variant)]
enum MyEnum {
    Data: u8,
    Empty1: (),
    Empty2: (),
    Empty3
}
  
fn main() {
    let _a = MyEnum::Empty1(());
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
    let _a = MyEnum::Tuple((1, 2));
}
"#;

const USER_DEFINED_UNIT_VARIANT: &str = r#"
type Unit = ();
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Empty: Unit,
}
  
fn main() {
    let _a = MyEnum::Empty(());
}
"#;

#[test]
fn multiple_empty_variants_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_EMPTY_VARIANTS, @r"
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:5:5
        Empty1: (), // Some comment
        ^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:6:5
        Empty2: (        ),         // Different comment
        ^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn correct_variant_diagnostics() {
    test_lint_diagnostics!(CORRECT_VARIANT, @"");
}

#[test]
fn mismatched_bracket_variant_diagnostics() {
    test_lint_diagnostics!(MISMATCHED_BRACKET_VARIANT, @r"
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:5:5
        Empty: () // Comment
        ^^^^^^^^^
    ");
}

#[test]
fn extra_brackets_variant_diagnostics() {
    test_lint_diagnostics!(EXTRA_BRACKETS_VARIANT, @"");
}

#[test]
fn allow_multiple_empty_variants_diagnostics() {
    test_lint_diagnostics!(ALLOW_MULTIPLE_EMPTY_VARIANTS, @r"");
}

#[test]
fn tuple_variant_diagnostics() {
    test_lint_diagnostics!(TUPLE_VARIANT, @"");
}

#[test]
fn user_defined_unit_variant_diagnostics() {
    test_lint_diagnostics!(USER_DEFINED_UNIT_VARIANT, @r"
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:6:5
        Empty: Unit,
        ^^^^^^^^^^^
    ");
}

#[test]
fn multiple_empty_variants_fixer() {
    test_lint_fixer!(MULTIPLE_EMPTY_VARIANTS, @r"
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty1, // Some comment
        Empty2,         // Different comment
        Empty3
    }
      
    fn main() {
        let _a = MyEnum::Empty1(   ( ) );
        let _b = MyEnum::Empty2((  ));
        let _c = MyEnum::Empty3;
    }
    ");
}

#[test]
fn mismatched_bracket_variant_fixer() {
    test_lint_fixer!(MISMATCHED_BRACKET_VARIANT, @r"
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty // Comment
    }
      
    fn main() {
        let _a = MyEnum::Empty;
    }
    ");
}

#[test]
fn user_defined_unit_variant_fixer() {
    test_lint_fixer!(USER_DEFINED_UNIT_VARIANT, @r"
    type Unit = ();
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty,
    }
      
    fn main() {
        let _a = MyEnum::Empty(());
    }
    ");
}

#[test]
fn extra_brackets_variant_fixer() {
    test_lint_fixer!(EXTRA_BRACKETS_VARIANT, @r"
    #[derive(Drop)]
    enum MyEnum {
        Data: u8,
        Empty
    }
      
    fn main() {
        let _a = MyEnum::Empty(()); 
    }
    ");
}

#[test]
fn allow_multiple_empty_variants_fixer() {
    test_lint_fixer!(ALLOW_MULTIPLE_EMPTY_VARIANTS, @r"
    #[derive(Drop)]
    #[allow(empty_enum_brackets_variant)]
    enum MyEnum {
        Data: u8,
        Empty1: (),
        Empty2: (),
        Empty3
    }
      
    fn main() {
        let _a = MyEnum::Empty1(());
        let _b = MyEnum::Empty2(());
        let _c = MyEnum::Empty3;
    }
    ");
}
