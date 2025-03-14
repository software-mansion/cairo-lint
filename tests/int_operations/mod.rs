use crate::{test_lint_diagnostics, test_lint_fixer};

const INT_GE_PLUS_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x >= y + 1 {}
}
"#;

const INT_GT_PLUS_ONE_ALLOWED: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    #[allow(int_ge_plus_one)]
    if x >= y + 1 {}
}
"#;

const INT_GT_PLUS_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x > y + 1 {}
}
"#;

const INT_GE_MIN_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x - 1 >= y {}
}
"#;

const INT_GT_MIN_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x - 1 > y {}
}
"#;

const INT_LE_PLUS_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x + 1 <= y {}
}
"#;

const INT_LE_PLUS_ONE_NOT: &str = r#"
fn f() -> u32 {
    2
}
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x + f() <= y {}
}
"#;

const INT_LT_PLUS_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x + 1 < y {}
}
"#;

const INT_LE_MIN_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x <= y - 1 {}
}
"#;

const INT_LT_MIN_ONE: &str = r#"
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x < y - 1 {}
}
"#;

#[test]
fn int_ge_plus_one_diagnostics() {
    test_lint_diagnostics!(INT_GE_PLUS_ONE, @r"
    warning: Plugin diagnostic: Unnecessary add operation in integer >= comparison. Use simplified comparison.
     --> lib.cairo:5:8
      |
    5 |     if x >= y + 1 {}
      |        ----------
      |
    ");
}

#[test]
fn int_ge_plus_one_fixer() {
    test_lint_fixer!(INT_GE_PLUS_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x > y {}
    }
    "#);
}

#[test]
fn int_gt_plus_one_allowed_diagnostics() {
    test_lint_diagnostics!(INT_GT_PLUS_ONE_ALLOWED, @r#"
    "#);
}

#[test]
fn int_gt_plus_one_allowed_fixer() {
    test_lint_fixer!(INT_GT_PLUS_ONE_ALLOWED, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        #[allow(int_ge_plus_one)]
        if x >= y + 1 {}
    }
    "#);
}

#[test]
fn int_gt_plus_one_diagnostics() {
    test_lint_diagnostics!(INT_GT_PLUS_ONE, @r#"
    "#);
}

#[test]
fn int_gt_plus_one_fixer() {
    test_lint_fixer!(INT_GT_PLUS_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x > y + 1 {}
    }
    "#);
}

#[test]
fn int_ge_min_one_diagnostics() {
    test_lint_diagnostics!(INT_GE_MIN_ONE, @r"
    warning: Plugin diagnostic: Unnecessary sub operation in integer >= comparison. Use simplified comparison.
     --> lib.cairo:5:8
      |
    5 |     if x - 1 >= y {}
      |        ----------
      |
    ");
}

#[test]
fn int_ge_min_one_fixer() {
    test_lint_fixer!(INT_GE_MIN_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x > y {}
    }
    "#);
}

#[test]
fn int_gt_min_one_diagnostics() {
    test_lint_diagnostics!(INT_GT_MIN_ONE, @r#"
    "#);
}

#[test]
fn int_gt_min_one_fixer() {
    test_lint_fixer!(INT_GT_MIN_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x - 1 > y {}
    }
    "#);
}

#[test]
fn int_le_plus_one_diagnostics() {
    test_lint_diagnostics!(INT_LE_PLUS_ONE, @r"
    warning: Plugin diagnostic: Unnecessary add operation in integer <= comparison. Use simplified comparison.
     --> lib.cairo:5:8
      |
    5 |     if x + 1 <= y {}
      |        ----------
      |
    ");
}

#[test]
fn int_le_plus_one_not_diagnostics() {
    test_lint_diagnostics!(INT_LE_PLUS_ONE_NOT, @r"");
}

#[test]
fn int_le_plus_one_fixer() {
    test_lint_fixer!(INT_LE_PLUS_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x < y {}
    }
    "#);
}

#[test]
fn int_lt_plus_one_diagnostics() {
    test_lint_diagnostics!(INT_LT_PLUS_ONE, @r#"
    "#);
}

#[test]
fn int_lt_plus_one_fixer() {
    test_lint_fixer!(INT_LT_PLUS_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x + 1 < y {}
    }
    "#);
}

#[test]
fn int_le_min_one_diagnostics() {
    test_lint_diagnostics!(INT_LE_MIN_ONE, @r"
    warning: Plugin diagnostic: Unnecessary sub operation in integer <= comparison. Use simplified comparison.
     --> lib.cairo:5:8
      |
    5 |     if x <= y - 1 {}
      |        ----------
      |
    ");
}

#[test]
fn int_le_min_one_fixer() {
    test_lint_fixer!(INT_LE_MIN_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x < y {}
    }
    "#);
}

#[test]
fn int_lt_min_one_diagnostics() {
    test_lint_diagnostics!(INT_LT_MIN_ONE, @r#"
    "#);
}

#[test]
fn int_lt_min_one_fixer() {
    test_lint_fixer!(INT_LT_MIN_ONE, @r#"
    fn main() {
        let x: u32 = 1;
        let y: u32 = 1;
        if x < y - 1 {}
    }
    "#);
}
