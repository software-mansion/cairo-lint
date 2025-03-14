use crate::{test_lint_diagnostics, test_lint_fixer};

const ONE_UNUSED_VARIABLE: &str = r#"
  fn main() {
    let a: Option<felt252> = Option::Some(1);
  }
"#;

const TWO_UNUSED_VARIABLES: &str = r#"
  fn main() {
    let a: Option<felt252> = Option::Some(1);
    let b = 1;
  }
"#;

const PLENTY_UNUSED_VARIABLES: &str = r#"
  fn main() {
    let used: Option<felt252> = Option::Some(1);
    let b = 1;
    {
        let c = 1_u32;
    }
    if true {
        let _avoid_collapsible = 1_u32;
        if false {
            let d = 3_u32;
        } else {
            let e = false;
        }
        let f: Array<u32> = array![];
    } else {
        let g: Option<u32> = Option::None;
        match used {
            Option::Some(not_used) => 1_u32,
            Option::None => 2_u32,
        };
    }
  }
"#;

#[test]
fn one_unused_variable_diagnostics() {
    test_lint_diagnostics!(ONE_UNUSED_VARIABLE, @r"
    warning: Unused variable. Consider ignoring by prefixing with `_`.
     --> lib.cairo:3:9
      |
    3 |     let a: Option<felt252> = Option::Some(1);
      |         -
      |
    ");
}

#[test]
fn one_unused_variable_fixer() {
    test_lint_fixer!(ONE_UNUSED_VARIABLE, @r"
    fn main() {
      let a: Option<felt252> = Option::Some(1);
    }
    ");
}

#[test]
fn two_unused_variables_diagnostics() {
    test_lint_diagnostics!(TWO_UNUSED_VARIABLES, @r"
    warning: Unused variable. Consider ignoring by prefixing with `_`.
     --> lib.cairo:3:9
      |
    3 |     let a: Option<felt252> = Option::Some(1);
      |         -
      |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
     --> lib.cairo:4:9
      |
    4 |     let b = 1;
      |         -
      |
    ");
}

#[test]
fn two_unused_variables_fixer() {
    test_lint_fixer!(TWO_UNUSED_VARIABLES, @r#"
    fn main() {
      let a: Option<felt252> = Option::Some(1);
      let b = 1;
    }
  "#);
}

#[test]
fn plenty_unused_variables_diagnostics() {
    test_lint_diagnostics!(PLENTY_UNUSED_VARIABLES, @r"
    warning: Unused variable. Consider ignoring by prefixing with `_`.
     --> lib.cairo:6:13
      |
    6 |         let c = 1_u32;
      |             -
      |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
      --> lib.cairo:11:17
       |
    11 |             let d = 3_u32;
       |                 -
       |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
      --> lib.cairo:13:17
       |
    13 |             let e = false;
       |                 -
       |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
      --> lib.cairo:15:13
       |
    15 |         let f: Array<u32> = array![];
       |             -
       |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
      --> lib.cairo:19:26
       |
    19 |             Option::Some(not_used) => 1_u32,
       |                          --------
       |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
      --> lib.cairo:17:13
       |
    17 |         let g: Option<u32> = Option::None;
       |             -
       |
    warning: Unused variable. Consider ignoring by prefixing with `_`.
     --> lib.cairo:4:9
      |
    4 |     let b = 1;
      |         -
      |
    ");
}

#[test]
fn plenty_unused_variables_fixer() {
    test_lint_fixer!(PLENTY_UNUSED_VARIABLES, @r#"
    fn main() {
      let used: Option<felt252> = Option::Some(1);
      let b = 1;
      {
          let c = 1_u32;
      }
      if true {
          let _avoid_collapsible = 1_u32;
          if false {
              let d = 3_u32;
          } else {
              let e = false;
          }
          let f: Array<u32> = array![];
      } else {
          let g: Option<u32> = Option::None;
          match used {
              Option::Some(not_used) => 1_u32,
              Option::None => 2_u32,
          };
      }
    }
  "#);
}
