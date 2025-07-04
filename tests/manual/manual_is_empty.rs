use crate::{test_lint_diagnostics, test_lint_fixer};

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_WITH_VAR: &str = r#"
fn main() {
    let a = array![];
    let _vl = if a.len() == 0 {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_MACRO: &str = r#"
fn main() {
    let _vl = if array![].len() == 0 {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_TRAIT: &str = r#"
fn main() {
    let _vl = if core::array::ArrayTrait::<felt252>::new().len() == 0 {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_ON_STRUCT_MEMBER: &str = r#"
struct B {
    pub ary: Array<felt252>
}

struct A {
    pub b: B
}

fn main() {
    let strct = A {
        b: B {
            ary: array![123],
        }
    };

    let _vl = if strct.b.ary.len() == 0 {
       true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_SPAN: &str = r#"
fn main() {
    let _vl = if array![].span().len() == 0 {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_FUNC_CALL_AND_SPAN: &str = r#"
fn get_array() -> Array<felt252> {
     array![1, 2, 3]
}

fn main() {
    let _vl = if get_array().span().len() == 0 {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO: &str = r#"
fn main() {
    let ref_v: Array<felt252> = array![];
    let _vl = if ref_v == array![] {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO_UNFORMATTED: &str = r#"
fn main() {
    let ref_v: Array<felt252> = array![];
    let _vl = if array![

    ] == ref_v {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_REF_CHECK_WITH_EMPTY_ARRAY_TRAIT: &str = r#"
fn main() {
    let ref_v: Array<felt252> = array![];
    let _vl = if ref_v == ArrayTrait::new() {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_FUNC_CALL_CHECK_WITH_EMPTY_ARRAY_TRAIT: &str = r#"
fn get_array() -> Array<felt252> {
     array![1, 2, 3]
}

fn main() {
    let _vl = if get_array() == core::array::ArrayTrait::new() {
        true
    } else {
        false
    };
}
"#;

const TEST_MANUAL_IS_EMPTY_LEN_CHECK_IN_WHILE_LOOP: &str = r#"
fn main() {
    let v: Array<felt252> = array![];
    while v.len() == 0 {}
}
"#;

const TEST_MANUAL_IS_EMPTY_CHECK_IN_WHILE_LOOP_WITH_EMPTY_ARRAY_MACRO: &str = r#"
fn main() {
    let v: Array<felt252> = array![];
    while v == array![] {}
}
"#;

#[test]
fn test_is_manual_empty_via_len_check_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_WITH_VAR, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:18
        let _vl = if a.len() == 0 {
                     ^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_via_len_check_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_WITH_VAR, @r"
    fn main() {
        let a = array![];
        let _vl = if a.is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_via_macro_create_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_MACRO, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:3:18
        let _vl = if array![].len() == 0 {
                     ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_via_macro_create_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_MACRO, @r"
    fn main() {
        let _vl = if array![].is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_basic_is_manual_empty_via_trait_create_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_TRAIT, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:3:18
        let _vl = if core::array::ArrayTrait::<felt252>::new().len() == 0 {
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_basic_is_manual_empty_via_trait_create_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_TRAIT, @r"
    fn main() {
        let _vl = if core::array::ArrayTrait::<felt252>::new().is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_on_struct_member_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_ON_STRUCT_MEMBER, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:17:18
        let _vl = if strct.b.ary.len() == 0 {
                     ^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_on_struct_member_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_ON_STRUCT_MEMBER, @r"
    struct B {
        pub ary: Array<felt252>,
    }

    struct A {
        pub b: B,
    }

    fn main() {
        let strct = A { b: B { ary: array![123] } };

        let _vl = if strct.b.ary.is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_on_func_call_and_span_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_FUNC_CALL_AND_SPAN, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:7:18
        let _vl = if get_array().span().len() == 0 {
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_on_func_call_and_span_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_FUNC_CALL_AND_SPAN, @r"
    fn get_array() -> Array<felt252> {
        array![1, 2, 3]
    }

    fn main() {
        let _vl = if get_array().span().is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_on_span_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_SPAN, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:3:18
        let _vl = if array![].span().len() == 0 {
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_on_span_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_DIRECT_VIA_SPAN, @r"
    fn main() {
        let _vl = if array![].span().is_empty() {
            true
        } else {
            false
        };
    }
    ")
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:18
        let _vl = if ref_v == array![] {
                     ^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO, @r"
    fn main() {
        let ref_v: Array<felt252> = array![];
        let _vl = if ref_v.is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_ref_with_empty_array_trait_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_REF_CHECK_WITH_EMPTY_ARRAY_TRAIT, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:18
        let _vl = if ref_v == ArrayTrait::new() {
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_ref_with_empty_array_trait_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_REF_CHECK_WITH_EMPTY_ARRAY_TRAIT, @r"
    fn main() {
        let ref_v: Array<felt252> = array![];
        let _vl = if ref_v.is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_func_call_with_empty_array_trait_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_FUNC_CALL_CHECK_WITH_EMPTY_ARRAY_TRAIT, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:7:18
        let _vl = if get_array() == core::array::ArrayTrait::new() {
                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_func_call_with_empty_array_trait_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_FUNC_CALL_CHECK_WITH_EMPTY_ARRAY_TRAIT, @r"
    fn get_array() -> Array<felt252> {
        array![1, 2, 3]
    }

    fn main() {
        let _vl = if get_array().is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_unformatted_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO_UNFORMATTED, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:18-6:14
          let _vl = if array![
     __________________^
    | 
    |     ] == ref_v {
    |______________^
    ");
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_unformatted_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_CHECK_WITH_EMPTY_ARRAY_MACRO_UNFORMATTED, @r"
    fn main() {
        let ref_v: Array<felt252> = array![];
        let _vl = if ref_v.is_empty() {
            true
        } else {
            false
        };
    }
    ");
}

#[test]
fn test_is_manual_empty_with_len_check_in_while_loop_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_IN_WHILE_LOOP, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:11
        while v.len() == 0 {}
              ^^^^^^^^^^^^
    ");
}
#[test]
fn test_is_manual_empty_with_len_check_in_while_loop_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_LEN_CHECK_IN_WHILE_LOOP, @r"
    fn main() {
        let v: Array<felt252> = array![];
        while v.is_empty() {}
    }
    ");
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_in_while_loop_diagnostics() {
    test_lint_diagnostics!(TEST_MANUAL_IS_EMPTY_CHECK_IN_WHILE_LOOP_WITH_EMPTY_ARRAY_MACRO, @r"
    Plugin diagnostic: Manual check for `is_empty` detected. Consider using `is_empty()` instead
     --> lib.cairo:4:11
        while v == array![] {}
              ^^^^^^^^^^^^^
    ");
}

#[test]
fn test_is_manual_empty_with_empty_array_macro_in_while_loop_fixer() {
    test_lint_fixer!(TEST_MANUAL_IS_EMPTY_CHECK_IN_WHILE_LOOP_WITH_EMPTY_ARRAY_MACRO, @r"
    fn main() {
        let v: Array<felt252> = array![];
        while v.is_empty() {}
    }
    ");
}
