use crate::{test_lint_diagnostics, test_lint_fixer};

const SINGLE_PANIC: &str = r#"
fn main() {
  panic!("panic");
}
"#;

const MULTIPLE_PANIC: &str = r#"
fn main() {
  panic!("panic");
  panic!("panic 2");
  panic!("panic 3");
}
"#;

const MULTIPLE_PANIC_AND_OTHER_MACROS: &str = r#"
fn main() {
  panic!("panic");
  panic!("panic 2");
  println!("print");
}
"#;

const EMPTY_PANIC: &str = r#"
fn main() {
  panic!("");
}
"#;

const EMPTY_PANIC_ALLOWED: &str = r#"
fn main() {
    #[allow(panic)]
    panic!("");
}
"#;

const EMPTY_PANIC_FUNCTION_ALLOWED: &str = r#"
fn main() {
    #[allow(panic)]
    panic(array![]);
}
"#;

const EMPTY_PANIC_FUNCTION: &str = r#"
fn main() {
    panic(array![]);
}
"#;

const NO_PANIC: &str = r#"
fn main() {
  println!("print");
}
"#;

const PANIC_INSIDE_FUNCTION: &str = r#"
pub fn print_name() {
  println!("Hello Alan");
  panic!("panic Alan");
}

fn main() {
  print_name();
}
"#;

const MULTIPLE_PANIC_ALLOWED_IN_FUNCTION: &str = r#"
#[allow(panic)]
fn main() {
    panic!("panic");
    panic!("panic 2");
    panic!("panic 3");
}
"#;

const PANIC_IN_TRAIT_FUNCTION: &str = r#"
#[derive(Drop)]
struct MyStruct {} 

trait TExample {
    fn risky_function(self: @MyStruct) {
        panic!("This is a panic in a trait function");
    }
}

impl Example of TExample {}

fn main() {
    let instance = MyStruct {}
    instance.risky_function();
}
"#;

const PANIC_WITH_ARGS: &str = r#"
fn main() {
    let a = 5;
    panic!("a shouldn't be equal to {}", a);
}
"#;

#[test]
fn single_panic_diagnostics() {
    test_lint_diagnostics!(SINGLE_PANIC, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:3:3
      panic!("panic");
      ^^^^^
    "#);
}

#[test]
fn single_panic_fixer() {
    test_lint_fixer!(SINGLE_PANIC, @r#"
    fn main() {
        panic!("panic");
    }
    "#);
}

#[test]
fn multiple_panic_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_PANIC, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:3:3
      panic!("panic");
      ^^^^^
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:4:3
      panic!("panic 2");
      ^^^^^
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:5:3
      panic!("panic 3");
      ^^^^^
    "#);
}

#[test]
fn multiple_panic_fixer() {
    test_lint_fixer!(MULTIPLE_PANIC, @r#"
    fn main() {
        panic!("panic");
        panic!("panic 2");
        panic!("panic 3");
    }
    "#);
}

#[test]
fn multiple_panic_and_other_macros_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_PANIC_AND_OTHER_MACROS, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:3:3
      panic!("panic");
      ^^^^^
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:4:3
      panic!("panic 2");
      ^^^^^
    "#);
}

#[test]
fn multiple_panic_and_other_macros_fixer() {
    test_lint_fixer!(MULTIPLE_PANIC_AND_OTHER_MACROS, @r#"
    fn main() {
        panic!("panic");
        panic!("panic 2");
        println!("print");
    }
    "#);
}

#[test]
fn empty_panic_diagnostics() {
    test_lint_diagnostics!(EMPTY_PANIC, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:3:3
      panic!("");
      ^^^^^
    "#);
}

#[test]
fn empty_panic_fixer() {
    test_lint_fixer!(EMPTY_PANIC, @r#"
    fn main() {
        panic!("");
    }
    "#);
}

#[test]
fn empty_panic_allowed_diagnostics() {
    test_lint_diagnostics!(EMPTY_PANIC_ALLOWED, @r#"
    "#);
}

#[test]
fn empty_panic_allowed_fixer() {
    test_lint_fixer!(EMPTY_PANIC_ALLOWED, @r#"
    fn main() {
        #[allow(panic)]
        panic!("");
    }
    "#);
}

#[test]
fn empty_panic_function_allowed_diagnostics() {
    test_lint_diagnostics!(EMPTY_PANIC_FUNCTION_ALLOWED, @r#"
    "#);
}

#[test]
fn empty_panic_function_allowed_fixer() {
    test_lint_fixer!(EMPTY_PANIC_FUNCTION_ALLOWED, @r#"
    fn main() {
        #[allow(panic)]
        panic(array![]);
    }
    "#);
}

#[test]
fn empty_panic_function_diagnostics() {
    test_lint_diagnostics!(EMPTY_PANIC_FUNCTION, @r"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:3:5
        panic(array![]);
        ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn empty_panic_function_fixer() {
    test_lint_fixer!(EMPTY_PANIC_FUNCTION, @r#"
    fn main() {
        panic(array![]);
    }
    "#);
}

#[test]
fn no_panic_diagnostics() {
    test_lint_diagnostics!(NO_PANIC, @r#"
    "#);
}

#[test]
fn no_panic_fixer() {
    test_lint_fixer!(NO_PANIC, @r#"
    fn main() {
        println!("print");
    }
    "#);
}

#[test]
fn panic_inside_function_diagnostics() {
    test_lint_diagnostics!(PANIC_INSIDE_FUNCTION, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:4:3
      panic!("panic Alan");
      ^^^^^
    "#);
}

#[test]
fn panic_inside_function_fixer() {
    test_lint_fixer!(PANIC_INSIDE_FUNCTION, @r#"
    pub fn print_name() {
        println!("Hello Alan");
        panic!("panic Alan");
    }

    fn main() {
        print_name();
    }
    "#);
}

#[test]
fn multiple_panic_allowed_in_function_diagnostics() {
    test_lint_diagnostics!(MULTIPLE_PANIC_ALLOWED_IN_FUNCTION, @"")
}

#[test]
fn multiple_panic_allowed_in_function_fixer() {
    test_lint_fixer!(MULTIPLE_PANIC_ALLOWED_IN_FUNCTION, @r##"
    #[allow(panic)]
    fn main() {
        panic!("panic");
        panic!("panic 2");
        panic!("panic 3");
    }
    "##)
}

#[test]
fn panic_in_trait_function_diagnostics() {
    test_lint_diagnostics!(PANIC_IN_TRAIT_FUNCTION, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:7:9
            panic!("This is a panic in a trait function");
            ^^^^^
    "#);
}

#[test]
fn panic_with_args_diagnostics() {
    test_lint_diagnostics!(PANIC_WITH_ARGS, @r#"
    Plugin diagnostic: Leaving `panic` in the code is discouraged.
     --> lib.cairo:4:5
        panic!("a shouldn't be equal to {}", a);
        ^^^^^
    "#);
}

#[test]
fn panic_with_args_fixer() {
    test_lint_fixer!(PANIC_WITH_ARGS, @r#"
    fn main() {
        let a = 5;
        panic!("a shouldn't be equal to {}", a);
    }
    "#);
}
