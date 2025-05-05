use crate::test_lint_diagnostics;

const SIMPLE_RETURN_UNIT_TYPE: &str = r#"
fn test_fn() -> () {
    ()
}

fn main() {
    println!("Hello, world!");
    test_fn()
}
"#;

const SIMPLE_RETURN_UNIT_TYPE_WITH_IMPLICIT_UNIT_TYPE: &str = r#"
fn test_fn() {
    ()
}

fn main() {
    println!("Hello, world!");
    test_fn()
}
"#;

const SIMPLE_RETURN_UNIT_TYPE_ALLOWED: &str = r#"
#[allow(unit_return_type)]
fn test_fn() -> () {
    ()
}
fn main() {
    println!("Hello, world!");
    test_fn()
}
"#;

const RETURN_UNIT_TYPE_IN_TRAIT: &str = r#"
trait MyTrait {
    fn test_fn() -> ();
}

fn main() {
    println!("Hello, world!");
}
"#;

const RETURN_UNIT_TYPE_IN_TRAIT_ALLOWED: &str = r#"
trait MyTrait {
    #[allow(unit_return_type)]
    fn test_fn() -> ();
}

fn main() {
    println!("Hello, world!");
}
"#;

const RETURN_UNIT_TYPE_IN_IMPL: &str = r#"
trait MyTrait {
    fn test_fn();
}

impl MyTraitImpl of MyTrait {
    fn test_fn() -> () {
        ()
    }
}

fn main() {
    println!("Hello, world!");
}
"#;

const RETURN_UNIT_TYPE_IN_IMPL_ALLOWED: &str = r#"
trait MyTrait {
    fn test_fn();
}

impl MyTraitImpl of MyTrait {
    #[allow(unit_return_type)]
    fn test_fn() -> () {
        ()
    }
}

fn main() {
    println!("Hello, world!");
}
"#;

#[test]
fn simple_return_unit_type_diagnostics() {
    test_lint_diagnostics!(SIMPLE_RETURN_UNIT_TYPE, @r"
    Plugin diagnostic: unnecessary declared unit return type `()`
     --> lib.cairo:2:11
    fn test_fn() -> () {
              ^^^^^^^^
    ");
}

#[test]
fn simple_return_unit_type_with_implicit_unit_type_diagnostics() {
    test_lint_diagnostics!(SIMPLE_RETURN_UNIT_TYPE_WITH_IMPLICIT_UNIT_TYPE, @r"");
}

#[test]
fn simple_return_unit_type_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_RETURN_UNIT_TYPE_ALLOWED, @r#""#);
}

#[test]
fn return_unit_type_in_trait_diagnostics() {
    test_lint_diagnostics!(RETURN_UNIT_TYPE_IN_TRAIT, @r"
  Plugin diagnostic: unnecessary declared unit return type `()`
   --> lib.cairo:3:15
      fn test_fn() -> ();
                ^^^^^^^^
  ");
}

#[test]
fn return_unit_type_in_trait_allowed_diagnostics() {
    test_lint_diagnostics!(RETURN_UNIT_TYPE_IN_TRAIT_ALLOWED, @r#""#);
}

#[test]
fn return_unit_type_in_impl_diagnostics() {
    test_lint_diagnostics!(RETURN_UNIT_TYPE_IN_IMPL, @r"
    Plugin diagnostic: unnecessary declared unit return type `()`
     --> lib.cairo:7:15
        fn test_fn() -> () {
                  ^^^^^^^^
    ");
}

#[test]
fn return_unit_type_in_impl_allowed_diagnostics() {
    test_lint_diagnostics!(RETURN_UNIT_TYPE_IN_IMPL_ALLOWED, @"");
}
