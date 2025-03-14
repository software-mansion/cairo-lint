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

#[test]
fn simple_destructuring_match_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5
      |
    4 | /     match variable {
    5 | |         Option::Some(a) => println!("{a}"),
    6 | |         _ => (),
    7 | |     };
      | |_____-
      |
    "#);
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
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_SCOPE, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5
      |
    4 | /     match variable {
    5 | |         Option::Some(a) => println!("{a}"),
    6 | |         _ => {},
    7 | |     };
      | |_____-
      |
    "#);
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
    test_lint_diagnostics!(SIMPLE_DESTRUCTURING_MATCH_WITH_UNIT_IN_SCOPE, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5
      |
    4 | /     match variable {
    5 | |         Option::Some(a) => println!("{a}"),
    6 | |         _ => { () },
    7 | |     };
      | |_____-
      |
    "#);
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
    test_lint_diagnostics!(NESTED_DESTRUCTURING_MATCH, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:6:28
      |
    6 |           Option::Some(a) => match a {
      |  ____________________________-
    7 | |             Option::Some(b) => println!("{b}"),
    8 | |             _ => (),
    9 | |         },
      | |_________-
      |
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
      --> lib.cairo:5:5
       |
     5 | /     match variable {
     6 | |         Option::Some(a) => match a {
    ...  |
    10 | |         _ => (),
    11 | |     };
       | |_____-
       |
    "#);
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
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
      --> lib.cairo:5:5
       |
     5 | /     match variable {
     6 | |         // This match is invalid hence no diag/fix for this one
    ...  |
    11 | |         _ => (),
    12 | |     };
       | |_____-
       |
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
    test_lint_diagnostics!(DESTRUCTURING_MATCH_TWISTED_DIFFERENTLY, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
      --> lib.cairo:8:28
       |
     8 |           Option::Some(a) => match a {
       |  ____________________________-
     9 | |             Option::Some(b) => println!("{b}"),
    10 | |             _ => (),
    11 | |         },
       | |_________-
       |
    "#);
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
    test_lint_diagnostics!(DESTRUCTURING_COMPREHENSIVE_MATCH, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for destructuring a single pattern. Consider using `if let`
     --> lib.cairo:4:5
      |
    4 | /     match variable {
    5 | |         Option::Some(a) => println!("{a}"),
    6 | |         Option::None => (),
    7 | |     };
      | |_____-
      |
    "#);
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
    test_lint_diagnostics!(COMPREHENSIVE_MATCH, @r#"
    warning: Plugin diagnostic: you seem to be trying to use `match` for an equality check. Consider using `if`
     --> lib.cairo:4:5
      |
    4 | /     match variable {
    5 | |         Option::None => println!("None"),
    6 | |         Option::Some => (),
    7 | |     };
      | |_____-
      |
    "#);
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
