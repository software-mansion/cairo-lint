use crate::{test_lint_diagnostics, test_lint_fixer};

const DUPLICATE_UNDERSCORE_ARGS_ALLOWED: &str = r#"
#[allow(duplicate_underscore_args)]
fn foo(a: u32, _a: u32) {}
"#;

const DUPLICATE_UNDERSCORE_ARGS2: &str = r#"
fn foo(c: u32, _c: u32) {}
"#;

const DUPLICATE_UNDERSCORE_LONGER_ARGS: &str = r#"
fn foo(test: u32, _test: u32) {}
"#;

const DUPLICATE_UNDERSCORE_LONGER_ARGS2: &str = r#"
fn foo(darth: u32, _darth: u32) {}
"#;

const DUPLICATE_UNDERSCORE_LONGER_ARGS3: &str = r#"
fn foo(stark: u32, _stark: u32) {}
"#;

const DUPLICATE_UNDERSCORE_LONGER_ARGS4: &str = r#"
fn foo(_test: u32, test: u32) {}
"#;

#[test]
fn duplicate_underscore_args_allowed_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_ARGS_ALLOWED, @r#"
    "#);
}

#[test]
fn duplicate_underscore_args_allowed_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_ARGS_ALLOWED, @r#"
    #[allow(duplicate_underscore_args)]
    fn foo(a: u32, _a: u32) {}
    "#);
}

#[test]
fn duplicate_underscore_args2_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_ARGS2, @r"
    warning: Plugin diagnostic: duplicate arguments, having another argument having almost the same name makes code comprehension and documentation more difficult
     --> lib.cairo:2:16
      |
    2 | fn foo(c: u32, _c: u32) {}
      |                --
      |
    ");
}

#[test]
fn duplicate_underscore_args2_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_ARGS2, @r#"
    fn foo(c: u32, _c: u32) {}
    "#);
}

#[test]
fn duplicate_underscore_longer_args_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_LONGER_ARGS, @r"
    warning: Plugin diagnostic: duplicate arguments, having another argument having almost the same name makes code comprehension and documentation more difficult
     --> lib.cairo:2:19
      |
    2 | fn foo(test: u32, _test: u32) {}
      |                   -----
      |
    ");
}

#[test]
fn duplicate_underscore_longer_args_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_LONGER_ARGS, @r#"
    fn foo(test: u32, _test: u32) {}
    "#);
}

#[test]
fn duplicate_underscore_longer_args2_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_LONGER_ARGS2, @r"
    warning: Plugin diagnostic: duplicate arguments, having another argument having almost the same name makes code comprehension and documentation more difficult
     --> lib.cairo:2:20
      |
    2 | fn foo(darth: u32, _darth: u32) {}
      |                    ------
      |
    ");
}

#[test]
fn duplicate_underscore_longer_args2_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_LONGER_ARGS2, @r#"
    fn foo(darth: u32, _darth: u32) {}
    "#);
}

#[test]
fn duplicate_underscore_longer_args3_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_LONGER_ARGS3, @r"
    warning: Plugin diagnostic: duplicate arguments, having another argument having almost the same name makes code comprehension and documentation more difficult
     --> lib.cairo:2:20
      |
    2 | fn foo(stark: u32, _stark: u32) {}
      |                    ------
      |
    ");
}

#[test]
fn duplicate_underscore_longer_args3_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_LONGER_ARGS3, @r#"
    fn foo(stark: u32, _stark: u32) {}
    "#);
}

#[test]
fn duplicate_underscore_longer_args4_diagnostics() {
    test_lint_diagnostics!(DUPLICATE_UNDERSCORE_LONGER_ARGS4, @r"
    warning: Plugin diagnostic: duplicate arguments, having another argument having almost the same name makes code comprehension and documentation more difficult
     --> lib.cairo:2:20
      |
    2 | fn foo(_test: u32, test: u32) {}
      |                    ----
      |
    ");
}

#[test]
fn duplicate_underscore_longer_args4_fixer() {
    test_lint_fixer!(DUPLICATE_UNDERSCORE_LONGER_ARGS4, @r#"
    fn foo(_test: u32, test: u32) {}
    "#);
}
