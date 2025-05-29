use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_DOUBLE_COMPARISON_ALLOWED: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    #[allow(simplifiable_comparison)]
    x == y || x > y 
}
"#;

const SIMPLE_LET_DOUBLE_COMPARISON_ALLOWED: &str = r#"
fn main() {
    let x = 5_u32;
    let y = 10_u32;
    #[allow(simplifiable_comparison)]
     let _cond = x == y || x > y;
}
"#;

const DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    // Just a comment here.
    if x == y || x > y {
        true
    } else {
        false
    }
}
"#;

const SIMPLIFIABLE_COMPARISON_ALLOWED: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    #[allow(simplifiable_comparison)]
    if x == y || x > y {
        true
    } else {
        false
    }
}
"#;

const CONTRADICTORY_COMPARISON_ALLOWED: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    #[allow(contradictory_comparison)]
    if x == y && x != y {
        true
    } else {
        false
    }
}
"#;

const REDUNDANT_COMPARISON_ALLOWED: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    #[allow(redundant_comparison)]
    if x == y && x >= y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_EQUAL_OR_LESS_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x == y || x < y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x > y || x == y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_GREATER_THAN_OR_LESS_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x > y || x < y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x >= y && x <= y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x < y || x == y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_LESS_THAN_OR_GREATER_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x < y || x > y {
        true
    } else {
        false
    }
}
"#;

const DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL_AND_GREATER_THAN_OR_EQUAL: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x <= y && x >= y {
        true
    } else {
        false
    }
}
"#;

const NOT_REDUNDANT_DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    let z = 15_u32;
    if x == y || z > y {
        true
    } else {
        false
    }
}
"#;

const CONTRADICTORY_LESS_THAN_AND_GREATER_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x < y && x > y {
        true
    } else {
        false
    }
}
"#;

const CONTRADICTORY_EQUAL_AND_LESS_THAN: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x == y && x < y {
        true
    } else {
        false
    }
}
"#;

const REDUNDANT_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x >= y || x <= y {
        true
    } else {
        false
    }
}
"#;

const IMPOSSIBLE_COMPARISON: &str = r#"
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x <= y && x > y {
        true
    } else {
        false
    }
}
"#;

const EVERY_IMPOSSIBLE_COMPARISON: &str = r#"
fn main() -> bool {
    let x = 4_u32;
    let y = 10_u32;
    if x > y && x >= y {
        true
    } else {
        false
    }
}
"#;

const IMPOSSIBLE_COMPARISON_GT_AND_LT: &str = r#"
fn main() {
    let x: u32 = 1;
    if x > 200 && x < 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_GT_AND_LT_POSSIBLE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x > 100 && x < 105 {
        //possible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_GT_AND_LE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x > 200 && x <= 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_GE_AND_LT: &str = r#"
fn main() {
    let x: u32 = 1;
    if x >= 200 && x < 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_GE_AND_LE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x >= 200 && x <= 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LT_AND_GT: &str = r#"
fn main() {
    let x: u32 = 1;
    if x < 100 && x > 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LT_AND_GT_POSSIBLE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x < 105 && x > 100 {
        //possible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LT_AND_GE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x < 100 && x >= 100 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LE_AND_GT: &str = r#"
fn main() {
    let x: u32 = 1;
    if x <= 100 && x > 200 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LE_AND_GE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x <= 100 && x >= 200 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LE_AND_GE_ALLOWED: &str = r#"
fn main() {
    let x: u32 = 1;
    #[allow(impossible_comparison)]
    if x <= 100 && x >= 200 {
        //impossible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_LE_AND_GE_DIFFERENT_VAR: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x <= 100 && y >= 200 {
        //possible to reach
    }
}
"#;

const IMPOSSIBLE_COMPARISON_WITH_ELSE_CLAUSE: &str = r#"
fn main() {
    let x: u32 = 1;
    if x >= 200 && x < 100 {
        //impossible to reach
    } else if x == 1 {
        //possible to reach
    } else {
        //possible to reach
    }
}
"#;

#[test]
fn simple_double_comparison_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_DOUBLE_COMPARISON_ALLOWED, @r#"
    "#);
}

#[test]
fn simple_double_comparison_allowed_fixer() {
    test_lint_fixer!(SIMPLE_DOUBLE_COMPARISON_ALLOWED, @r"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        #[allow(simplifiable_comparison)]
        x == y || x > y
    }
    ");
}

#[test]
fn simple_let_double_comparison_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LET_DOUBLE_COMPARISON_ALLOWED, @r#"
    "#);
}

#[test]
fn simple_let_double_comparison_allowed_fixer() {
    test_lint_fixer!(SIMPLE_LET_DOUBLE_COMPARISON_ALLOWED, @r"
    fn main() {
        let x = 5_u32;
        let y = 10_u32;
        #[allow(simplifiable_comparison)]
        let _cond = x == y || x > y;
    }
    ");
}

#[test]
fn double_comparison_equal_or_greater_than_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:6:8
        if x == y || x > y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_equal_or_greater_than_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN, @r"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        // Just a comment here.
        if x >= y {
            true
        } else {
            false
        }
    }
    ");
}

#[test]
fn simplifiable_comparison_allowed_diagnostics() {
    test_lint_diagnostics!(SIMPLIFIABLE_COMPARISON_ALLOWED, @r#"
    "#);
}

#[test]
fn simplifiable_comparison_allowed_fixer() {
    test_lint_fixer!(SIMPLIFIABLE_COMPARISON_ALLOWED, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        #[allow(simplifiable_comparison)]
        if x == y || x > y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn contradictory_comparison_allowed_diagnostics() {
    test_lint_diagnostics!(CONTRADICTORY_COMPARISON_ALLOWED, @r#"
    "#);
}

#[test]
fn contradictory_comparison_allowed_fixer() {
    test_lint_fixer!(CONTRADICTORY_COMPARISON_ALLOWED, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        #[allow(contradictory_comparison)]
        if x == y && x != y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn redundant_comparison_allowed_diagnostics() {
    test_lint_diagnostics!(REDUNDANT_COMPARISON_ALLOWED, @r#"
    "#);
}

#[test]
fn redundant_comparison_allowed_fixer() {
    test_lint_fixer!(REDUNDANT_COMPARISON_ALLOWED, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        #[allow(redundant_comparison)]
        if x == y && x >= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_equal_or_less_than_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_EQUAL_OR_LESS_THAN, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:5:8
        if x == y || x < y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_equal_or_less_than_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_EQUAL_OR_LESS_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x <= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_greater_than_or_equal_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:5:8
        if x > y || x == y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_greater_than_or_equal_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x >= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_greater_than_or_less_than_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_GREATER_THAN_OR_LESS_THAN, @r"
    Plugin diagnostic: Redundant double comparison found. Consider simplifying to a single comparison.
     --> lib.cairo:5:8
        if x > y || x < y {
           ^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_greater_than_or_less_than_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_GREATER_THAN_OR_LESS_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x != y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_greater_than_or_equal_and_less_than_or_equal_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:5:8
        if x >= y && x <= y {
           ^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_greater_than_or_equal_and_less_than_or_equal_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x == y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_less_than_or_equal_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:5:8
        if x < y || x == y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_less_than_or_equal_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x <= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_less_than_or_greater_than_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_LESS_THAN_OR_GREATER_THAN, @r"
    Plugin diagnostic: Redundant double comparison found. Consider simplifying to a single comparison.
     --> lib.cairo:5:8
        if x < y || x > y {
           ^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_less_than_or_greater_than_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_LESS_THAN_OR_GREATER_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x != y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn double_comparison_less_than_or_equal_and_greater_than_or_equal_diagnostics() {
    test_lint_diagnostics!(DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL_AND_GREATER_THAN_OR_EQUAL, @r"
    Plugin diagnostic: This double comparison can be simplified.
     --> lib.cairo:5:8
        if x <= y && x >= y {
           ^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn double_comparison_less_than_or_equal_and_greater_than_or_equal_fixer() {
    test_lint_fixer!(DOUBLE_COMPARISON_LESS_THAN_OR_EQUAL_AND_GREATER_THAN_OR_EQUAL, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x == y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn not_redundant_double_comparison_equal_or_greater_than_diagnostics() {
    test_lint_diagnostics!(NOT_REDUNDANT_DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN, @r#"
    "#);
}

#[test]
fn not_redundant_double_comparison_equal_or_greater_than_fixer() {
    test_lint_fixer!(NOT_REDUNDANT_DOUBLE_COMPARISON_EQUAL_OR_GREATER_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        let z = 15_u32;
        if x == y || z > y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn contradictory_less_than_and_greater_than_diagnostics() {
    test_lint_diagnostics!(CONTRADICTORY_LESS_THAN_AND_GREATER_THAN, @r"
    Plugin diagnostic: This double comparison is contradictory and always false.
     --> lib.cairo:5:8
        if x < y && x > y {
           ^^^^^^^^^^^^^^
    ");
}

#[test]
fn contradictory_less_than_and_greater_than_fixer() {
    test_lint_fixer!(CONTRADICTORY_LESS_THAN_AND_GREATER_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x < y && x > y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn contradictory_equal_and_less_than_diagnostics() {
    test_lint_diagnostics!(CONTRADICTORY_EQUAL_AND_LESS_THAN, @r"
    Plugin diagnostic: This double comparison is contradictory and always false.
     --> lib.cairo:5:8
        if x == y && x < y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn contradictory_equal_and_less_than_fixer() {
    test_lint_fixer!(CONTRADICTORY_EQUAL_AND_LESS_THAN, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x == y && x < y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn redundant_greater_than_or_equal_and_less_than_or_equal_diagnostics() {
    test_lint_diagnostics!(REDUNDANT_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL, @r"
    Plugin diagnostic: Redundant double comparison found. Consider simplifying to a single comparison.
     --> lib.cairo:5:8
        if x >= y || x <= y {
           ^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn redundant_greater_than_or_equal_and_less_than_or_equal_fixer() {
    test_lint_fixer!(REDUNDANT_GREATER_THAN_OR_EQUAL_AND_LESS_THAN_OR_EQUAL, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x >= y || x <= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn impossible_comparison_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON, @r"
    Plugin diagnostic: This double comparison is contradictory and always false.
     --> lib.cairo:5:8
        if x <= y && x > y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON, @r#"
    fn main() -> bool {
        let x = 5_u32;
        let y = 10_u32;
        if x <= y && x > y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn every_impossible_comparison_diagnostics() {
    test_lint_diagnostics!(EVERY_IMPOSSIBLE_COMPARISON, @r"
    Plugin diagnostic: This double comparison is contradictory and always false.
     --> lib.cairo:5:8
        if x > y && x >= y {
           ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn every_impossible_comparison_fixer() {
    test_lint_fixer!(EVERY_IMPOSSIBLE_COMPARISON, @r#"
    fn main() -> bool {
        let x = 4_u32;
        let y = 10_u32;
        if x > y && x >= y {
            true
        } else {
            false
        }
    }
    "#);
}

#[test]
fn impossible_comparison_gt_and_lt_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_GT_AND_LT, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x > 200 && x < 100 {
           ^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_gt_and_lt_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_GT_AND_LT, @r"
    fn main() {
        let x: u32 = 1;
        if x > 200 && x < 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_gt_and_lt_possible_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_GT_AND_LT_POSSIBLE, @r#"
    "#);
}

#[test]
fn impossible_comparison_gt_and_lt_possible_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_GT_AND_LT_POSSIBLE, @r"
    fn main() {
        let x: u32 = 1;
        if x > 100 && x < 105 {//possible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_gt_and_le_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_GT_AND_LE, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x > 200 && x <= 100 {
           ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_gt_and_le_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_GT_AND_LE, @r"
    fn main() {
        let x: u32 = 1;
        if x > 200 && x <= 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_ge_and_lt_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_GE_AND_LT, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x >= 200 && x < 100 {
           ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_ge_and_lt_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_GE_AND_LT, @r"
    fn main() {
        let x: u32 = 1;
        if x >= 200 && x < 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_ge_and_le_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_GE_AND_LE, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x >= 200 && x <= 100 {
           ^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_ge_and_le_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_GE_AND_LE, @r"
    fn main() {
        let x: u32 = 1;
        if x >= 200 && x <= 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_lt_and_gt_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LT_AND_GT, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x < 100 && x > 100 {
           ^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_lt_and_gt_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LT_AND_GT, @r"
    fn main() {
        let x: u32 = 1;
        if x < 100 && x > 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_lt_and_gt_possible_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LT_AND_GT_POSSIBLE, @r#"
    "#);
}

#[test]
fn impossible_comparison_lt_and_gt_possible_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LT_AND_GT_POSSIBLE, @r"
    fn main() {
        let x: u32 = 1;
        if x < 105 && x > 100 {//possible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_lt_and_ge_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LT_AND_GE, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x < 100 && x >= 100 {
           ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_lt_and_ge_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LT_AND_GE, @r"
    fn main() {
        let x: u32 = 1;
        if x < 100 && x >= 100 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_le_and_gt_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LE_AND_GT, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x <= 100 && x > 200 {
           ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_le_and_gt_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LE_AND_GT, @r"
    fn main() {
        let x: u32 = 1;
        if x <= 100 && x > 200 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_le_and_ge_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LE_AND_GE, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x <= 100 && x >= 200 {
           ^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_le_and_ge_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LE_AND_GE, @r"
    fn main() {
        let x: u32 = 1;
        if x <= 100 && x >= 200 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_le_and_ge_allowed_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LE_AND_GE_ALLOWED, @r#"
    "#);
}

#[test]
fn impossible_comparison_le_and_ge_allowed_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LE_AND_GE_ALLOWED, @r"
    fn main() {
        let x: u32 = 1;
        #[allow(impossible_comparison)]
        if x <= 100 && x >= 200 {//impossible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_le_and_ge_different_var_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_LE_AND_GE_DIFFERENT_VAR, @r#"
    "#);
}

#[test]
fn impossible_comparison_le_and_ge_different_var_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_LE_AND_GE_DIFFERENT_VAR, @r"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x <= 100 && y >= 200 {//possible to reach
        }
    }
    ");
}

#[test]
fn impossible_comparison_with_else_clause_diagnostics() {
    test_lint_diagnostics!(IMPOSSIBLE_COMPARISON_WITH_ELSE_CLAUSE, @r"
    Plugin diagnostic: Impossible condition, always false
     --> lib.cairo:4:8
        if x >= 200 && x < 100 {
           ^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn impossible_comparison_with_else_clause_fixer() {
    test_lint_fixer!(IMPOSSIBLE_COMPARISON_WITH_ELSE_CLAUSE, @r"
    fn main() {
        let x: u32 = 1;
        if x >= 200 && x < 100 {//impossible to reach
        } else if x == 1 {//possible to reach
        } else {//possible to reach
        }
    }
    ");
}
