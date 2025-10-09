use super::test_lint_diagnostics;

#[test]
fn redundant_into_not_triggered() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) -> u256 {
        x.into()
    }
    "#,
    @r""
    );
}

#[test]
fn redundant_into_tail() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) -> u128 {
        x.into()
    }
    "#,
    @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:5
        x.into()
        ^^^^^^^^
    "
    );
}

#[test]
fn redundant_into_expr() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _v: bool = 352_u128 == x.into();
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:32
        let _v: bool = 352_u128 == x.into();
                                   ^^^^^^^^
    ");
}

#[test]
fn redundant_into_variable() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _v: u128 = x.into();
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:20
        let _v: u128 = x.into();
                       ^^^^^^^^
    ");
}

#[test]
fn redundant_into_direct_trait_call() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _v: u128 = Into::<u128>::into(x);
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:20
        let _v: u128 = Into::<u128>::into(x);
                       ^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_into_ambiguous_output() {
    test_lint_diagnostics!(r#"
    #[derive(Drop)]
    struct MyStruct {
        nested: felt252
    }

    fn f(x: u128) {
        let _v = MyStruct {
            nested: 123,
         }.into();
    }
    "#, @r"
    Trait `core::traits::Into::<test::MyStruct, ?0>` has multiple implementations, in: `core::option::TIntoOption::<test::MyStruct>`, `core::traits::TIntoT::<test::MyStruct>`
     --> lib.cairo:9:8
         }.into();
           ^^^^
    ");
}

#[test]
fn redundant_into_ambiguous_input() {
    test_lint_diagnostics!(r#"
    fn ambiguous_input<I>() {
        let num = get_number::<I>();
        let _result: u8 = num.into();
    }

    fn get_number<I>() -> I {
        panic!("ehhh")
    }
    "#, @r#"
    Trait has no implementation in context: core::traits::Into::<I, core::integer::u8>.
     --> lib.cairo:3:27
        let _result: u8 = num.into();
                              ^^^^
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:7:5
        panic!("ehhh")
        ^^^^^
    "#)
}

#[test]
fn redundant_into_reference_arg() {
    test_lint_diagnostics!(r#"
    #[derive(Copy, Drop)]
    struct MyStructA {
        value: felt252
    }

    fn f(ref refka_my_struct_a: MyStructA) {
        let _v: MyStructA  = Into::<MyStructA>::into(refka_my_struct_a);
    }
    "#,@r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:7:26
        let _v: MyStructA  = Into::<MyStructA>::into(refka_my_struct_a);
                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    "
    );
}

#[test]
fn redundant_try_into_tail_option() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) -> Option<u128> {
        x.try_into()
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:5
        x.try_into()
        ^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_try_into_tail_unwrap() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) -> u128 {
        x.try_into().unwrap()
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:5
        x.try_into().unwrap()
        ^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_try_into_variable() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _varka: u128 = x.try_into().unwrap();
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:24
        let _varka: u128 = x.try_into().unwrap();
                           ^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_try_into_expr() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _v: bool = 352_u128 == x.try_into().unwrap();
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:32
        let _v: bool = 352_u128 == x.try_into().unwrap();
                                   ^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_try_into_direct_trait_call() {
    test_lint_diagnostics!(r#"
    fn f(x: u128) {
        let _v: u128 = TryInto::<u128>::try_into(x).unwrap();
    }
    "#, @r"
    Plugin diagnostic: Redundant conversion: input and output types are the same.
     --> lib.cairo:2:20
        let _v: u128 = TryInto::<u128>::try_into(x).unwrap();
                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ")
}

#[test]
fn redundant_try_into_ambiguous_output() {
    test_lint_diagnostics!(r#"
    #[derive(Drop)]
    struct MyStruct {
        nested: felt252
    }

    fn f(x: u128) {
        let _v = MyStruct {
            nested: 123,
         }.try_into().unwrap();
    }
    "#, @r"
    Type annotations needed. Failed to infer ?2.
     --> lib.cairo:9:8
         }.try_into().unwrap();
           ^^^^^^^^
    ");
}

#[test]
fn redundant_try_into_ambiguous_input() {
    test_lint_diagnostics!(r#"
    fn ambiguous_input<I>() {
        let num = get_number::<I>();
        let _result: u8 = num.try_into().unwrap();
    }

    fn get_number<I>() -> I {
        panic!("ehhh")
    }
    "#, @r#"
    Trait has no implementation in context: core::traits::TryInto::<I, core::integer::u8>.
     --> lib.cairo:3:27
        let _result: u8 = num.try_into().unwrap();
                              ^^^^^^^^
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:7:5
        panic!("ehhh")
        ^^^^^
    "#);
}
