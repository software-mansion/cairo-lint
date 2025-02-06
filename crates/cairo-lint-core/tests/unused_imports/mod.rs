use crate::{test_lint_diagnostics, test_lint_fixer};

const SINGLE_UNUSED_IMPORT: &str = r#"
use core::integer::u128_safe_divmod;
fn main() {
}
"#;

const MULTIPLE_UNUSED_IMPORTS: &str = r#"
use core::integer::{u128_safe_divmod, u128_byte_reverse};
fn main() {
}
"#;

const UNUSED_IMPORT_ALIASED: &str = r#"
use core::integer::u128_safe_divmod as foo;
fn main() {
}
"#;

const UNUSED_IMPORT_TRAIT: &str = r#"
use core::num::traits::OverflowingAdd;
fn main() {
}
"#;

const MULTI_WITH_ONE_USED_AND_ONE_UNUSED: &str = r#"
use core::integer::{u128_safe_divmod, u128_byte_reverse};
fn main() {
    u128_byte_reverse(10_u128);
}
"#;

const MIX_OF_MULTI_AND_LEAF_IMPORTS_IN_A_SINGLE_STATEMENT: &str = r#"
use core::{
    integer::{u128_safe_divmod, u128_byte_reverse},
    option::Option,
};

fn main() {
    let _ = Option::<u128>::Some(5);
}
"#;

const MULTIPLE_IMPORT_STATEMENTS_LINES_WITH_SOME_USED_AND_SOME_UNUSED: &str = r#"
use core::option::Option;
use core::array::ArrayTrait;
use core::box::BoxTrait;

fn main() {
    let _ = Option::<u128>::Some(5);
    let _res = BoxTrait::<u128>::new(5);
}
"#;

#[test]
fn single_unused_import_diagnostics() {
    test_lint_diagnostics!(SINGLE_UNUSED_IMPORT, @r"
    warning: Unused import: `test::u128_safe_divmod`
     --> lib.cairo:2:20
      |
    2 | use core::integer::u128_safe_divmod;
      |                    ----------------
      |
    ");
}

#[test]
fn single_unused_import_fixer() {
    test_lint_fixer!(SINGLE_UNUSED_IMPORT, @r#"
    fn main() {
    }
    "#);
}

#[test]
fn multiple_unused_imports_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_UNUSED_IMPORTS, @r"
    warning: Unused import: `test::u128_safe_divmod`
     --> lib.cairo:2:21
      |
    2 | use core::integer::{u128_safe_divmod, u128_byte_reverse};
      |                     ----------------
      |
    warning: Unused import: `test::u128_byte_reverse`
     --> lib.cairo:2:39
      |
    2 | use core::integer::{u128_safe_divmod, u128_byte_reverse};
      |                                       -----------------
      |
    ");
}

#[test]
fn multiple_unused_imports_fixer() {
    test_lint_fixer!(MULTIPLE_UNUSED_IMPORTS, @r#"
    fn main() {
    }
    "#);
}

#[test]
fn unused_import_aliased_diagnostics() {
    test_lint_diagnostics!(UNUSED_IMPORT_ALIASED, @r"
    warning: Unused import: `test::foo`
     --> lib.cairo:2:20
      |
    2 | use core::integer::u128_safe_divmod as foo;
      |                    -----------------------
      |
    ");
}

#[test]
fn unused_import_aliased_fixer() {
    test_lint_fixer!(UNUSED_IMPORT_ALIASED, @r#"
    fn main() {
    }
    "#);
}

#[test]
fn unused_import_trait_diagnostics() {
    test_lint_diagnostics!(UNUSED_IMPORT_TRAIT, @r"
    warning: Unused import: `test::OverflowingAdd`
     --> lib.cairo:2:24
      |
    2 | use core::num::traits::OverflowingAdd;
      |                        --------------
      |
    ");
}

#[test]
fn unused_import_trait_fixer() {
    test_lint_fixer!(UNUSED_IMPORT_TRAIT, @r#"
    fn main() {
    }
    "#);
}

#[test]
fn multi_with_one_used_and_one_unused_diagnostics() {
    test_lint_diagnostics!(MULTI_WITH_ONE_USED_AND_ONE_UNUSED, @r"
    warning: Unused import: `test::u128_safe_divmod`
     --> lib.cairo:2:21
      |
    2 | use core::integer::{u128_safe_divmod, u128_byte_reverse};
      |                     ----------------
      |
    ");
}

#[test]
fn multi_with_one_used_and_one_unused_fixer() {
    test_lint_fixer!(MULTI_WITH_ONE_USED_AND_ONE_UNUSED, @r#"
    use core::integer::u128_byte_reverse;
    fn main() {
        u128_byte_reverse(10_u128);
    }
    "#);
}

#[test]
fn mix_of_multi_and_leaf_imports_in_a_single_statement_diagnostics() {
    test_lint_diagnostics!(MIX_OF_MULTI_AND_LEAF_IMPORTS_IN_A_SINGLE_STATEMENT, @r"
    warning: Unused import: `test::u128_safe_divmod`
     --> lib.cairo:3:15
      |
    3 |     integer::{u128_safe_divmod, u128_byte_reverse},
      |               ----------------
      |
    warning: Unused import: `test::u128_byte_reverse`
     --> lib.cairo:3:33
      |
    3 |     integer::{u128_safe_divmod, u128_byte_reverse},
      |                                 -----------------
      |
    ");
}

#[test]
fn mix_of_multi_and_leaf_imports_in_a_single_statement_fixer() {
    test_lint_fixer!(MIX_OF_MULTI_AND_LEAF_IMPORTS_IN_A_SINGLE_STATEMENT, @r#"
    use core::option::Option;

    fn main() {
        let _ = Option::<u128>::Some(5);
    }
    "#);
}

#[test]
fn multiple_import_statements_lines_with_some_used_and_some_unused_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_IMPORT_STATEMENTS_LINES_WITH_SOME_USED_AND_SOME_UNUSED, @r"
    warning: Unused import: `test::ArrayTrait`
     --> lib.cairo:3:18
      |
    3 | use core::array::ArrayTrait;
      |                  ----------
      |
    ");
}

#[test]
fn multiple_import_statements_lines_with_some_used_and_some_unused_fixer() {
    test_lint_fixer!(MULTIPLE_IMPORT_STATEMENTS_LINES_WITH_SOME_USED_AND_SOME_UNUSED, @r#"
    use core::option::Option;
    use core::box::BoxTrait;

    fn main() {
        let _ = Option::<u128>::Some(5);
        let _res = BoxTrait::<u128>::new(5);
    }
    "#);
}
