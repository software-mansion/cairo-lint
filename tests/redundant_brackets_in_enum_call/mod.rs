use crate::{test_lint_diagnostics, test_lint_fixer};

const BASIC_REDUNDANT_BRACKETS: &str = r#"
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

const SIMPLE_UNIT_GENERIC: &str = r#"
#[derive(Drop)]
enum MyEnum< T> {
    V:   T,
    Value: u8,
    Empty: ()
}

fn main() {
    // Non-redundant parentheses (required arguments)
    let _a = MyEnum::V(());
    let _b = MyEnum::<()>::Value(1);
    
    // Redundant parentheses
    let _c = MyEnum::<()>::Empty(());
}
"#;

const REDUNDANT_UNIT_WITH_GENERIC_IN_PATH: &str = r#"
fn main() {
    // Redundant parentheses
    let _a = Option::<(  )>::Some(());
    let _b = Result::<( ), Option::<()>>::Ok(());
    
    // Non-redundant parentheses (required argument)
    let _c = Result::<( ), felt252>::Err('Hello');
}
"#;

const GENERIC_NAMED_ARGUMENT: &str = r#"
#[derive(Drop)]
enum MyEnum< T, E> {
    V:   T,
    Value: u8,
    V2: E
}
    
fn main() {
    // Redundant parentheses 
    let _a = MyEnum::<T: (), E: ()>::V(());
    let _d = MyEnum::<(), E: ()>::V(());

    // Non-redundant parentheses 
    let _b = MyEnum::<E : ()>::V(());
    let _c = MyEnum::<T: _, E: ()>::V(());
    let _d = MyEnum::<_, E: ()>::V(());
}
"#;

const NESTED_ENUM_VARIANTS: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Empty,
    Data: u8,
}

fn main() {
    // Non-redundant parentheses (required arguments)
    let _a = Result::<Option::<()>, ()>::Ok(Option::Some(()));
    
    // Redundant parentheses
    let _b = Option::Some(MyEnum::Empty(()));
    let _c = Option::Some(Option::Some(MyEnum::Empty(())));
}
"#;

const COMPLEX_NESTED_GENERICS: &str = r#"
#[derive(Drop)]
enum ComplexEnum<A, B, C> {
    Variant: (A, B, C),
    Empty,
}

fn main() {
    // Redundant parentheses
    let _a = ComplexEnum::<(), (), ()>::Empty(());
    
    // Non-redundant parentheses (required arguments)
    let _b = ComplexEnum::<(), (), ()>::Variant(((), (), ()));
}
"#;

const ENUM_IN_CONTAINERS: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Empty,
    Data: u8,
}

#[derive(Drop)]
struct MyStruct {
    field: MyEnum,
}

fn main() {
    // Redundant parentheses in array
    let _arr = [MyEnum::Empty(()), MyEnum::Data(1)];

    // Redundant parentheses in struct initialization
    let _s = MyStruct { field: MyEnum::Empty(()) };

    // Redundant parentheses in tuple
    let _t = (MyEnum::Empty(()), MyEnum::Data(1));
}
"#;

const ENUM_WITH_MODULE_PATHS: &str = r#"
mod my_module {
    #[derive(Drop)]
    pub enum MyEnum {
        Empty,
    }

    pub mod nested {
        #[derive(Drop)]
        pub enum NestedEnum {
            Empty,
        }
    }
}

fn main() {
    // Redundant parentheses with module paths
    let _a = my_module::MyEnum::Empty(());
    let _b = my_module::nested::NestedEnum::Empty(());
}
"#;

const ENUM_WITH_UNIT_RETURNING_EXPR: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Empty,
}

fn returns_unit() -> () { () }

fn main() {
    // Redundant parentheses with function call
    let _a = MyEnum::Empty(returns_unit());
    
    // Redundant parentheses with block expression
    let _b = MyEnum::Empty({
        let _ = 1;
        ()
    });
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

const CORRECT_TUPLE_VARIANT: &str = r#"
#[derive(Drop)]
enum MyEnum {
    Data: u8,
    Tuple: (u8, u8),
}

fn main() {
    // Non-redundant parentheses (required tuple argument)
    let _a = MyEnum::Tuple((1,2));
}
"#;

const CORRECT_MULTIPLE_OPTION_VARIANTS: &str = r#"
fn main() {
    // Non-redundant parentheses (required unit argument)
    let _a: Option<()> = Option::Some(());
    let _b: Option<_> = Option::Some(());
    let _c = Option::Some(());

    // No parentheses
    let _d = Option::<(  )>::Some;
}
"#;

const CORRECT_UNIT_OK_RESULT: &str = r#"
fn main() {
    let _a: Result<(), ()> = Result::Ok(());
}
"#;

const CORRECT_UNIT_ERR_RESULT: &str = r#"
fn main() {
    let _a: Result<(), ()> = Result::Err(());
}
"#;

const CORRECT_GENERIC_ARG_UNDERSCORE: &str = r#"
#[derive(Drop)]
enum MyEnum< T, E> {
    V:   T,
    Value: u8,
    V2: E
}
    
fn main() {
    let _a = MyEnum::<_, ()>::V(());
    let _b = MyEnum::<(), _>::V2(());
    let _c = Result::<(), _>::Err(());
}
"#;

const CORRECT_UNIT_GENERIC_AND_ALIAS: &str = r#"
type number = u8;

#[derive(Drop)]
enum MyEnum<T> {
    V: T,
    Value: number,
}

fn main() {
    let _a = MyEnum::V(());
    let _b = MyEnum::<()>::Value(1);
}
"#;

#[test]
fn redundant_bracket_call_diagnostics() {
    test_lint_diagnostics!(BASIC_REDUNDANT_BRACKETS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:9:14
        let _a = MyEnum::Empty(()); 
                 ^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_bracket_call_fixer() {
    test_lint_fixer!(BASIC_REDUNDANT_BRACKETS, @r"
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
fn simple_unit_generic_diagnostics() {
    test_lint_diagnostics!(SIMPLE_UNIT_GENERIC, @r"
    Plugin diagnostic: redundant parentheses in enum variant definition
     --> lib.cairo:6:5
        Empty: ()
        ^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:15:14
        let _c = MyEnum::<()>::Empty(());
                 ^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn simple_unit_fixer() {
    test_lint_fixer!(SIMPLE_UNIT_GENERIC, @r"
    #[derive(Drop)]
    enum MyEnum< T> {
        V:   T,
        Value: u8,
        Empty
    }

    fn main() {
        // Non-redundant parentheses (required arguments)
        let _a = MyEnum::V(());
        let _b = MyEnum::<()>::Value(1);
        
        // Redundant parentheses
        let _c = MyEnum::<()>::Empty;
    }
    ");
}

#[test]
fn unit_generic_in_path_diagnostics() {
    test_lint_diagnostics!(REDUNDANT_UNIT_WITH_GENERIC_IN_PATH, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:4:14
        let _a = Option::<(  )>::Some(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:5:14
        let _b = Result::<( ), Option::<()>>::Ok(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn unit_generic_in_path_fixer() {
    test_lint_fixer!(REDUNDANT_UNIT_WITH_GENERIC_IN_PATH, @r"
    fn main() {
        // Redundant parentheses
        let _a = Option::<(  )>::Some;
        let _b = Result::<( ), Option::<()>>::Ok;
        
        // Non-redundant parentheses (required argument)
        let _c = Result::<( ), felt252>::Err('Hello');
    }
    ")
}

#[test]
fn generic_named_argument_diagnostics() {
    test_lint_diagnostics!(GENERIC_NAMED_ARGUMENT, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:11:14
        let _a = MyEnum::<T: (), E: ()>::V(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:12:14
        let _d = MyEnum::<(), E: ()>::V(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn generic_named_argument_fixer() {
    test_lint_fixer!(GENERIC_NAMED_ARGUMENT, @r"
    #[derive(Drop)]
    enum MyEnum< T, E> {
        V:   T,
        Value: u8,
        V2: E
    }
        
    fn main() {
        // Redundant parentheses 
        let _a = MyEnum::<T: (), E: ()>::V;
        let _d = MyEnum::<(), E: ()>::V;

        // Non-redundant parentheses 
        let _b = MyEnum::<E : ()>::V(());
        let _c = MyEnum::<T: _, E: ()>::V(());
        let _d = MyEnum::<_, E: ()>::V(());
    }
    ")
}

#[test]
fn nested_enum_variants_diagnostics() {
    test_lint_diagnostics!(NESTED_ENUM_VARIANTS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:13:27
        let _b = Option::Some(MyEnum::Empty(()));
                              ^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:14:40
        let _c = Option::Some(Option::Some(MyEnum::Empty(())));
                                           ^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn nested_enum_variants_fixer() {
    test_lint_fixer!(NESTED_ENUM_VARIANTS, @r"
    #[derive(Drop)]
    enum MyEnum {
        Empty,
        Data: u8,
    }

    fn main() {
        // Non-redundant parentheses (required arguments)
        let _a = Result::<Option::<()>, ()>::Ok(Option::Some(()));
        
        // Redundant parentheses
        let _b = Option::Some(MyEnum::Empty);
        let _c = Option::Some(Option::Some(MyEnum::Empty));
    }
    ");
}

#[test]
fn complex_nested_generics_diagnostics() {
    test_lint_diagnostics!(COMPLEX_NESTED_GENERICS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:10:14
        let _a = ComplexEnum::<(), (), ()>::Empty(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn complex_nested_generics_fixer() {
    test_lint_fixer!(COMPLEX_NESTED_GENERICS, @r"
    #[derive(Drop)]
    enum ComplexEnum<A, B, C> {
        Variant: (A, B, C),
        Empty,
    }

    fn main() {
        // Redundant parentheses
        let _a = ComplexEnum::<(), (), ()>::Empty;
        
        // Non-redundant parentheses (required arguments)
        let _b = ComplexEnum::<(), (), ()>::Variant(((), (), ()));
    }
    ");
}

#[test]
fn enum_with_other_features_diagnostics() {
    test_lint_diagnostics!(ENUM_IN_CONTAINERS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:15:17
        let _arr = [MyEnum::Empty(()), MyEnum::Data(1)];
                    ^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:18:32
        let _s = MyStruct { field: MyEnum::Empty(()) };
                                   ^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:21:15
        let _t = (MyEnum::Empty(()), MyEnum::Data(1));
                  ^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn enum_with_other_features_fixer() {
    test_lint_fixer!(ENUM_IN_CONTAINERS, @r"
    #[derive(Drop)]
    enum MyEnum {
        Empty,
        Data: u8,
    }

    #[derive(Drop)]
    struct MyStruct {
        field: MyEnum,
    }

    fn main() {
        // Redundant parentheses in array
        let _arr = [MyEnum::Empty, MyEnum::Data(1)];

        // Redundant parentheses in struct initialization
        let _s = MyStruct { field: MyEnum::Empty(()) };

        // Redundant parentheses in tuple
        let _t = (MyEnum::Empty, MyEnum::Data(1));
    }
    ");
}

#[test]
fn enum_with_module_paths_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_MODULE_PATHS, @r"
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:18:14
        let _a = my_module::MyEnum::Empty(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: redundant parentheses in enum call
     --> lib.cairo:19:14
        let _b = my_module::nested::NestedEnum::Empty(());
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn enum_with_module_paths_fixer() {
    test_lint_fixer!(ENUM_WITH_MODULE_PATHS, @r"
    mod my_module {
        #[derive(Drop)]
        pub enum MyEnum {
            Empty,
        }

        pub mod nested {
            #[derive(Drop)]
            pub enum NestedEnum {
                Empty,
            }
        }
    }

    fn main() {
        // Redundant parentheses with module paths
        let _a = my_module::MyEnum::Empty;
        let _b = my_module::nested::NestedEnum::Empty;
    }
    ");
}

#[test]
fn enum_with_unit_returning_expr_diagnostics() {
    test_lint_diagnostics!(ENUM_WITH_UNIT_RETURNING_EXPR, @"");
}

#[test]
fn allow_multiple_empty_variants_diagnostics() {
    test_lint_diagnostics!(ALLOW_MULTIPLE_REDUNDANT_BRACKETS, @r"");
}

#[test]
fn correct_tuple_variant_diagnostics() {
    test_lint_diagnostics!(CORRECT_TUPLE_VARIANT, @r"");
}

#[test]
fn correct_multiple_option_variants_diagnostics() {
    test_lint_diagnostics!(CORRECT_MULTIPLE_OPTION_VARIANTS, @r"");
}

#[test]
fn correct_unit_ok_diagnostics() {
    test_lint_diagnostics!(CORRECT_UNIT_OK_RESULT, @r"");
}

#[test]
fn correct_unit_err_diagnostics() {
    test_lint_diagnostics!(CORRECT_UNIT_ERR_RESULT, @r"");
}

#[test]
fn correct_generic_arg_underscore_diagnostics() {
    test_lint_diagnostics!(CORRECT_GENERIC_ARG_UNDERSCORE, @r"");
}

#[test]
fn correct_unit_generic_and_alias_diagnostics() {
    test_lint_diagnostics!(CORRECT_UNIT_GENERIC_AND_ALIAS, @r"");
}
