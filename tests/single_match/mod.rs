use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_DESTRUCTURING_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        _ => (),
    };
}
"#;

const SIMPLE_DESTRUCTURING_MATCH_SECOND_ARM: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    // Invalid match
    match variable {
        _ => (),
        Option::Some(a) => println!("{a}"),
    };
}

"#;

const SIMPLE_DESTRUCTURING_MATCH_WITH_SCOPE: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        _ => {},
    };
}
"#;

const SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_IN_SCOPE: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        _ => { () },
    };
}
"#;

const NESTED_DESTRUCTURING_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(Option::Some(1_felt252));
    // Both valid
    match variable {
        Option::Some(a) => match a {
            Option::Some(b) => println!("{b}"),
            _ => (),
        },
        _ => (),
    };
}
"#;

const DESTRUCTURING_MATCH_TWISTED: &str = r#"
fn main() {
    let variable = Option::Some(Option::Some(1_felt252));
    // This comment should be omitted.
    match variable {
        // This match is invalid hence no diag/fix for this one
        Option::Some(a) => match a {
            _ => (),
            Option::Some(b) => println!("{b}"),
        },
        _ => (),
    };
}
"#;

const DESTRUCTURING_MATCH_TWISTED_DIFFERENTLY: &str = r#"
fn main() {
    let variable = Option::Some(Option::Some(1_felt252));
    // Invalid match so no diag/fix for this one
    match variable {
        _ => (),
        // This one is valid
        Option::Some(a) => match a {
            Option::Some(b) => println!("{b}"),
            _ => (),
        },
    };
}
"#;

const DESTRUCTURING_MATCH_SECOND_ARM: &str = r#"
fn main() {
    // Both of the match are invalid hence the plugin doesn't output any diag
    let variable = Option::Some(Option::Some(1_felt252));
    match variable {
        _ => (),
        Option::Some(a) => match a {
            _ => (),
            Option::Some(b) => println!("{b}"),
        },
    };
}
"#;

const DESTRUCTURING_COMPREHENSIVE_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        Option::None => (),
    };
}
"#;

const REVERSED_DESTRUCTURING_COMPREHENSIVE_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::None => (),
        Option::Some(a) => println!("{a}"),
    };
}
"#;

const SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_AND_COMMENT_IN_SCOPE: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        _ => { 
            // This is a comment
            () 
        },
    };
}
"#;

const SIMPLE_DESTRUCTURING_MATCH_WITH_COMMENT_IN_SCOPE: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::Some(a) => println!("{a}"),
        _ => { 
            // This is a comment
        },
    };
}
"#;

const COMPREHENSIVE_MATCH: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    match variable {
        Option::None => println!("None"),
        Option::Some => (),
    };
}
"#;

const COMPREHENSIVE_MATCH_ALLOWED: &str = r#"
fn main() {
    let variable = Option::Some(1_felt252);
    #[allow(equality_match)]
    match variable {
        Option::None => println!("None"),
        Option::Some => (),
    };
}
"#;

const DESTRUCTING_MATCH_IN_TRAIT: &str = r#"
#[derive(Drop)]
struct MyStruct {
    variable: Option::<felt252>,
}

trait TExample {
    fn match_struct(self: @MyStruct) {
        match *self.variable {
            Option::Some(a) => println!("{a}"),
            _ => { () },
        };  
    }
}

impl Example of TExample {}

fn main() {
    let instance = MyStruct { variable: Option::Some(1_felt252) };
    instance.match_struct();
}
"#;

#[test]
fn simple_destructuring_match_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5-7:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn simple_destructuring_match_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        if let Option::Some(a) = variable {
            println!("{a}")
        };
    }
    "#);
}

#[test]
fn simple_destructuring_match_second_arm_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_SECOND_ARM, @r#"
    "#);
}

#[test]
fn simple_destructuring_match_second_arm_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH_SECOND_ARM, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        // Invalid match
        match variable {
            _ => (),
            Option::Some(a) => println!("{a}"),
        };
    }
    "#);
}

#[test]
fn simple_destructuring_match_with_scope_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_SCOPE, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5-7:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn simple_destructuring_match_with_scope_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH_WITH_SCOPE, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        if let Option::Some(a) = variable {
            println!("{a}")
        };
    }
    "#);
}

#[test]
fn simple_destructuring_match_with_unit_in_scope_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_IN_SCOPE, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5-7:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn simple_destructuring_match_with_unit_in_scope_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_IN_SCOPE, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        if let Option::Some(a) = variable {
            println!("{a}")
        };
    }
    "#);
}

#[test]
fn nested_destructuring_match_diagnostics() {
    test_lint_diagnostics!(NESTED_DESTRUCTURING_MATCH, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:6:28-9:9
              Option::Some(a) => match a {
     ____________________________^
    | ...
    |         },
    |_________^
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:5:5-11:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn nested_destructuring_match_fixer() {
    test_lint_fixer!(NESTED_DESTRUCTURING_MATCH, @r#"
    Contains nested diagnostics can't fix it
    "#, true);
}

#[test]
fn destructuring_match_twisted_diagnostics() {
    test_lint_diagnostics!(DESTRUCTURING_MATCH_TWISTED, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:5:5-12:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn destructuring_match_twisted_fixer() {
    test_lint_fixer!(DESTRUCTURING_MATCH_TWISTED, @r#"
    fn main() {
        let variable = Option::Some(Option::Some(1_felt252));
        // This match is invalid hence no diag/fix for this one
        if let Option::Some(a) = variable {
            match a {
                _ => (),
                Option::Some(b) => println!("{b}"),
            }
        };
    }
    "#);
}

#[test]
fn destructuring_match_twisted_differently_diagnostics() {
    test_lint_diagnostics!(DESTRUCTURING_MATCH_TWISTED_DIFFERENTLY, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:8:28-11:9
              Option::Some(a) => match a {
     ____________________________^
    | ...
    |         },
    |_________^
    ");
}

#[test]
fn destructuring_match_twisted_differently_fixer() {
    test_lint_fixer!(DESTRUCTURING_MATCH_TWISTED_DIFFERENTLY, @r#"
    fn main() {
        let variable = Option::Some(Option::Some(1_felt252));
        // Invalid match so no diag/fix for this one
        match variable {
            _ => (),
            // This one is valid
            Option::Some(a) => if let Option::Some(b) = a {
        println!("{b}")
    },
        };
    }
    "#);
}

#[test]
fn destructuring_match_second_arm_diagnostics() {
    test_lint_diagnostics!(DESTRUCTURING_MATCH_SECOND_ARM, @r#"
    "#);
}

#[test]
fn destructuring_match_second_arm_fixer() {
    test_lint_fixer!(DESTRUCTURING_MATCH_SECOND_ARM, @r#"
    fn main() {
        // Both of the match are invalid hence the plugin doesn't output any diag
        let variable = Option::Some(Option::Some(1_felt252));
        match variable {
            _ => (),
            Option::Some(a) => match a {
                _ => (),
                Option::Some(b) => println!("{b}"),
            },
        };
    }
    "#);
}

#[test]
fn destructuring_comprehensive_match_diagnostics() {
    test_lint_diagnostics!(DESTRUCTURING_COMPREHENSIVE_MATCH, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5-7:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn destructuring_comprehensive_match_fixer() {
    test_lint_fixer!(DESTRUCTURING_COMPREHENSIVE_MATCH, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        if let Option::Some(a) = variable {
            println!("{a}")
        };
    }
    "#);
}

#[test]
fn reversed_destructuring_comprehensive_match_diagnostics() {
    test_lint_diagnostics!(REVERSED_DESTRUCTURING_COMPREHENSIVE_MATCH, @r#"
    "#);
}

#[test]
fn reversed_destructuring_comprehensive_match_fixer() {
    test_lint_fixer!(REVERSED_DESTRUCTURING_COMPREHENSIVE_MATCH, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        match variable {
            Option::None => (),
            Option::Some(a) => println!("{a}"),
        };
    }
    "#);
}

#[test]
fn simple_destructuring_match_with_unit_and_comment_in_scope_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_AND_COMMENT_IN_SCOPE, @r#"
    "#);
}

#[test]
fn simple_destructuring_match_with_unit_and_comment_in_scope_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_AND_COMMENT_IN_SCOPE, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        match variable {
            Option::Some(a) => println!("{a}"),
            _ => { 
                // This is a comment
                () 
            },
        };
    }
    "#);
}

#[test]
fn simple_destructuring_match_with_comment_in_scope_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_COMMENT_IN_SCOPE, @r#"
    "#);
}

#[test]
fn simple_destructuring_match_with_comment_in_scope_fixer() {
    test_lint_fixer!(SIMPLE_DESTRUCTURING_MATCH_WITH_COMMENT_IN_SCOPE, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        match variable {
            Option::Some(a) => println!("{a}"),
            _ => { 
                // This is a comment
            },
        };
    }
    "#);
}

#[test]
fn comprehensive_match_diagnostics() {
    test_lint_diagnostics!(COMPREHENSIVE_MATCH, @r"
    Plugin diagnostic: you seem to be trying to use `match` for an equality check. Consider using `if`
     --> lib.cairo:4:5-7:5
          match variable {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn comprehensive_match_fixer() {
    test_lint_fixer!(COMPREHENSIVE_MATCH, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        match variable {
            Option::None => println!("None"),
            Option::Some => (),
        };
    }
    "#);
}

#[test]
fn comprehensive_match_allowed_diagnostics() {
    test_lint_diagnostics!(COMPREHENSIVE_MATCH_ALLOWED, @r#"
    "#);
}

#[test]
fn comprehensive_match_allowed_fixer() {
    test_lint_fixer!(COMPREHENSIVE_MATCH_ALLOWED, @r#"
    fn main() {
        let variable = Option::Some(1_felt252);
        #[allow(equality_match)]
        match variable {
            Option::None => println!("None"),
            Option::Some => (),
        };
    }
    "#);
}

#[test]
fn destructing_match_in_trait_diagnostics() {
    test_lint_diagnostics!(DESTRUCTING_MATCH_IN_TRAIT, @r"
    Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:9:9-12:9
              match *self.variable {
     _________^
    | ...
    |         };  
    |_________^
    ");
}

#[test]
fn destructing_match_in_trait_fixer() {
    test_lint_fixer!(DESTRUCTING_MATCH_IN_TRAIT, @r##"
    #[derive(Drop)]
    struct MyStruct {
        variable: Option::<felt252>,
    }

    trait TExample {
        fn match_struct(self: @MyStruct) {
            if let Option::Some(a) = *self.variable {
                println!("{a}")
            };  
        }
    }

    impl Example of TExample {}

    fn main() {
        let instance = MyStruct { variable: Option::Some(1_felt252) };
        instance.match_struct();
    }
    "##);
}
