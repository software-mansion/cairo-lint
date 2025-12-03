use crate::{test_lint_diagnostics, test_lint_fixer};

const COLLAPSIBLE_MATCH_BASIC: &str = r#"
fn func(opt: Option<Result<u64, felt252>>) {
    let _n = match opt {
        Some(n) => match n {
            Ok(n) => Some(n),
            _ => None,
        },
        None => None,
    };
}
"#;

const COLLAPSIBLE_MATCH_DIFFERENT_ORDER_OF_ARMS: &str = r#"
fn func(opt: Option<Result<u64, felt252>>) {
    let _n = match opt {
        None => None,
        Some(n) => match n {
            _ => None,
            Ok(n) => Some(n),
        },
    };
}
"#;

const COLLAPSIBLE_MATCH_MIXED_ORDER_OF_ARMS: &str = r#"
fn func(opt: Option<Result<u64, felt252>>) {
    let _n = match opt {
        None => None,
        Some(n) => match n {
            Ok(n) => Some(n),
            _ => None,
        },
    };
}
"#;

const NON_COLLAPSIBLE_MATCH: &str = r#"
fn func(opt: Option<Result<u64, felt252>>) {
    let _n = match opt {
        Some(n) => match n {
            Ok(n) => Some(n),
            _ => Some(0),
        },
        None => None,
    };
}
"#;

#[test]
fn test_collapsible_match_basic_diagnostics() {
    test_lint_diagnostics!(
        COLLAPSIBLE_MATCH_BASIC, @r"
    Plugin diagnostic: Nested `match` statements can be collapsed into a single `match` statement.
     --> lib.cairo:3:14-9:5
          let _n = match opt {
     ______________^
    | ...
    |     };
    |_____^
    "
    );
}

#[test]
fn test_collapsible_match_basic_fixer() {
    test_lint_fixer!(
        COLLAPSIBLE_MATCH_BASIC,
        @r"
    fn func(opt: Option<Result<u64, felt252>>) {
        let _n = match opt {
            Some(Ok(n)) => Some(n),
            _ => None,
        };
    }
    "
    );
}

#[test]
fn test_collapsible_match_different_order_of_arms_diagnostics() {
    test_lint_diagnostics!(
        COLLAPSIBLE_MATCH_DIFFERENT_ORDER_OF_ARMS, @r"
    Plugin diagnostic: Nested `match` statements can be collapsed into a single `match` statement.
     --> lib.cairo:3:14-9:5
          let _n = match opt {
     ______________^
    | ...
    |     };
    |_____^
    "
    );
}

#[test]
fn test_collapsible_match_different_order_of_arms_fixer() {
    test_lint_fixer!(
        COLLAPSIBLE_MATCH_DIFFERENT_ORDER_OF_ARMS,
        @r"
    fn func(opt: Option<Result<u64, felt252>>) {
        let _n = match opt {
            Some(Ok(n)) => Some(n),
            _ => None,
        };
    }
    "
    );
}

#[test]
fn test_collapsible_match_mixed_order_of_arms_diagnostics() {
    test_lint_diagnostics!(
        COLLAPSIBLE_MATCH_MIXED_ORDER_OF_ARMS, @r"
    Plugin diagnostic: Nested `match` statements can be collapsed into a single `match` statement.
     --> lib.cairo:3:14-9:5
          let _n = match opt {
     ______________^
    | ...
    |     };
    |_____^
    "
    );
}

#[test]
fn test_collapsible_match_mixed_order_of_arms_fixer() {
    test_lint_fixer!(
        COLLAPSIBLE_MATCH_MIXED_ORDER_OF_ARMS,
        @r"
    fn func(opt: Option<Result<u64, felt252>>) {
        let _n = match opt {
            Some(Ok(n)) => Some(n),
            _ => None,
        };
    }
    "
    );
}

#[test]
fn test_non_collapsible_match_diagnostics() {
    test_lint_diagnostics!(
        NON_COLLAPSIBLE_MATCH, @""
    );
}

#[test]
fn test_non_collapsible_fixer() {
    test_lint_fixer!(
        NON_COLLAPSIBLE_MATCH,
        @r"
    fn func(opt: Option<Result<u64, felt252>>) {
        let _n = match opt {
            Some(n) => match n {
                Ok(n) => Some(n),
                _ => Some(0),
            },
            None => None,
        };
    }
    "
    );
}
