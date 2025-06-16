use crate::{test_lint_diagnostics, test_lint_fixer};

const ABC: &str = r#"
struct Abc {
    pub a: felt252,
}

#[generate_trait]
impl AbcImpl of AbcTrait {
    fn new() -> Abc {
        loop {
            break ();
        }
        Abc { a: 0 }
    }
}

fn main() {
    loop {
        break ();
    }
}
"#;

#[test]
fn abc_diagnostics() {
    test_lint_diagnostics!(ABC, @r"
    Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.
     --> lib.cairo:10:13
                break ();
                ^^^^^^^^^
    Plugin diagnostic: unnecessary double parentheses found after break. Consider removing them.
     --> lib.cairo:18:9
            break ();
            ^^^^^^^^^
    ");
}

#[test]
fn abc_fixer() {
    test_lint_fixer!(ABC, @r"
    struct Abc {
        pub a: felt252,
    }

    #[generate_trait]
    impl AbcImpl of AbcTrait {
        fn new() -> Abc {
            loop {
                break;
            }
            Abc { a: 0 }
        }
    }

    fn main() {
        loop {
            break;
        }
    }
    ");
}
