use crate::{test_lint_diagnostics, test_lint_fixer};

const IF_LET_WITH_DROPPABLE_TYPE: &str = r#"
// This type can be dropped => `manual_unwrap_or` will trigger instead.
#[derive(Drop)]
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    if let Option::Some(v) = a {
        v
    } else {
        Struct { x: 0x1 }
    };
}
"#;

const MATCH_WITH_DROPPABLE_TYPE: &str = r#"
// This type can be dropped => `manual_unwrap_or` will trigger instead.
#[derive(Drop)]
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    match a {
        Option::Some(v) => v,
        Option::None => Struct { x: 0x1 }
    };
}
"#;

const IF_LET_WITH_NON_DROPPABLE_TYPE: &str = r#"
// This type cannot be dropped => `manual_unwrap_or_else` will trigger.
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    if let Option::Some(v) = a {
        v
    } else {
        Struct { x: 0x1 }
    };
}
"#;

const IF_LET_WITH_BOOLEAN_EXPRESSION: &str = r#"
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    if true && let Option::Some(v) = a {
        v
    } else {
        Struct { x: 0x1 }
    };
}
"#;

const MATCH_WITH_NON_DROPPABLE_TYPE: &str = r#"
// This type cannot be dropped => `manual_unwrap_or_else` will trigger.
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    match a {
        Option::Some(v) => v,
        Option::None => Struct { x: 0x1 }
    };
}
"#;

const MATCH_WITH_REVERSED_ARMS_OPTION: &str = r#"
// This type cannot be dropped => `manual_unwrap_or_else` will trigger.
struct Struct {
    x: felt252,
}

fn main() {
    let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

    match a {
        Option::None => Struct { x: 0x1 },
        Option::Some(v) => v
    };
}
"#;

const MATCH_WITH_REVERSED_ARMS_RESULT: &str = r#"
// This type cannot be dropped => `manual_unwrap_or_else` will trigger.
struct Struct {
    x: felt252,
}

fn main() {
    let a: Result<Struct, felt252> = Result::Ok(Struct { x: 0x0 });

    let _ = match a {
        Result::Err(_) => Struct { x: 0x1 },
        Result::Ok(v) => v
    };
}
"#;

#[test]
fn if_let_with_droppable_type_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_DROPPABLE_TYPE, @r"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:11:5-15:5
          if let Option::Some(v) = a {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn if_let_with_droppable_type_fixer() {
    test_lint_fixer!(IF_LET_WITH_DROPPABLE_TYPE, @r"
    // This type can be dropped => `manual_unwrap_or` will trigger instead.
    #[derive(Drop)]
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        a.unwrap_or(Struct { x: 0x1 });
    }
    ");
}

#[test]
fn match_with_droppable_type_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_DROPPABLE_TYPE, @r"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:11:5-14:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn match_with_droppable_type_fixer() {
    test_lint_fixer!(MATCH_WITH_DROPPABLE_TYPE, @r"
    // This type can be dropped => `manual_unwrap_or` will trigger instead.
    #[derive(Drop)]
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        a.unwrap_or(Struct { x: 0x1 });
    }
    ");
}

#[test]
fn if_let_with_non_droppable_type_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_NON_DROPPABLE_TYPE, @r"
    Plugin diagnostic: Manual `unwrap_or_else` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:10:5-14:5
          if let Option::Some(v) = a {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn if_let_with_non_droppable_type_fixer() {
    test_lint_fixer!(IF_LET_WITH_NON_DROPPABLE_TYPE, @r"
    // This type cannot be dropped => `manual_unwrap_or_else` will trigger.
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        a.unwrap_or_else(|| Struct { x: 0x1 });
    }
    ");
}

#[test]
fn if_let_with_boolean_expression_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_BOOLEAN_EXPRESSION, @r"
    ");
}

#[test]
fn if_let_with_boolean_expression_fixer() {
    test_lint_fixer!(IF_LET_WITH_BOOLEAN_EXPRESSION, @r"
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        if true && let Option::Some(v) = a {
            v
        } else {
            Struct { x: 0x1 }
        };
    }
    ");
}

#[test]
fn match_with_non_droppable_type_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_NON_DROPPABLE_TYPE, @r"
    Plugin diagnostic: Manual `unwrap_or_else` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:10:5-13:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn match_with_non_droppable_type_fixer() {
    test_lint_fixer!(MATCH_WITH_NON_DROPPABLE_TYPE, @r"
    // This type cannot be dropped => `manual_unwrap_or_else` will trigger.
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        a.unwrap_or_else(|| Struct { x: 0x1 });
    }
    ");
}

#[test]
fn match_with_reversed_arms_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_REVERSED_ARMS_OPTION, @r"
    Plugin diagnostic: Manual `unwrap_or_else` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:10:5-13:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn match_with_reversed_arms_option_fixer() {
    test_lint_fixer!(MATCH_WITH_REVERSED_ARMS_OPTION, @r"
    // This type cannot be dropped => `manual_unwrap_or_else` will trigger.
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Option<Struct> = Option::Some(Struct { x: 0x0 });

        a.unwrap_or_else(|| Struct { x: 0x1 });
    }
    ");
}

#[test]
fn match_with_reversed_arms_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_REVERSED_ARMS_RESULT, @r"
    Plugin diagnostic: Manual `unwrap_or_else` detected. Consider using `unwrap_or_else()` instead.
     --> lib.cairo:10:13-13:5
          let _ = match a {
     _____________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn match_with_reversed_arms_result_fixer() {
    test_lint_fixer!(MATCH_WITH_REVERSED_ARMS_RESULT, @r"
    // This type cannot be dropped => `manual_unwrap_or_else` will trigger.
    struct Struct {
        x: felt252,
    }

    fn main() {
        let a: Result<Struct, felt252> = Result::Ok(Struct { x: 0x0 });

        let _ = a.unwrap_or_else(|| Struct { x: 0x1 });
    }
    ");
}
