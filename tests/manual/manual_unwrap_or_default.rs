use crate::{test_lint_diagnostics, test_lint_fixer};

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DEFAULT: &str = r#"
fn main() {
  let a: Option<ByteArray> = Option::Some("Helok");
  // This is just a variable.
  if let Option::Some(v) = a {
    v
   } else {
     Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_EMPTY_STRING: &str = r#"
fn main() {
  let x: Option<ByteArray> = Option::Some("Hello");
  // This is just a variable.
  if let Option::Some(v) = x {
    v
   } else {
     ""
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_NEW: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Option::Some(v) = x {
    v
   } else {
     ArrayTrait::new()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ZERO_INTEGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  if let Option::Some(v) = x {
    v
   } else {
    0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_FIXED_ARRAY: &str = r#"
fn main() {
  let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Option::Some(v) = a {
    v
   } else {
    [0; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE: &str = r#"
fn main() {
  let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
  // This is just a variable.
  if let Option::Some(v) = a {
    v
   } else {
      ("", 0, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ARRAY: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Option::Some(v) = x {
    v
   } else {
     array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_COMMENTS: &str = r#"
fn main() {
  let a: Option<ByteArray> = Option::Some("Helok");
  // This is just a variable.
  if let Option::Some(v) = a {
    // testing with comments
    v
   } else {
    // testing with comments
    Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE_WITHOUT_TRIGGER: &str = r#"
fn main() {
  let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
  // This is just a variable.
  if let Option::Some(v) = a {
    v
   } else {
      ("", 0, true)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let a: Option<ByteArray> = Option::Some("Helok");
  // This is just a variable.
  if let Option::Some(_) = a {
    100
   } else {
    0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITHOUT_TRIGGER: &str = r#"
fn main() {
  let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Option::Some(v) = a {
    v
   } else {
    [3; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE_WITHOUT_TRIGGER: &str = r#"
fn main() {
  let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ("sdkfh", 898, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ZERO_INTEGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => 0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_EMPTY_STRING: &str = r#"
fn main() {
  let x: Option<ByteArray> = Option::Some("Hello");
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ""
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DEFAULT: &str = r#"
fn main() {
  let a: Option<felt252> = Option::Some(1);
  // Somethings wrong.
  match a {
    Option::Some(v) => v,
    Option::None => Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_NEW: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ArrayTrait::new()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_FIXED_ARRAY: &str = r#"
fn main() {
  let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => [0; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE: &str = r#"
fn main() {
  let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ("", 0, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ARRAY: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_COMMENTS: &str = r#"
fn main() {
  let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => {
      // Testing with comments
      v
    },
    Option::None => {
      // Testing with comments
      [0; 5]
    }
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  match x {
    Option::Some(_) => array![1, 2, 3, 4, 5],
    Option::None => array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_WITHOUT_TRIGGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => 6778
  };
}
"#;

#[test]
fn manual_unwrap_or_default_for_if_let_with_default_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DEFAULT, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = a {
    6 | |     v
    7 | |    } else {
    8 | |      Default::default()
    9 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DEFAULT, @r#"
    fn main() {
      let a: Option<ByteArray> = Option::Some("Helok");
      // This is just a variable.
      a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_EMPTY_STRING, @r#"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = x {
    6 | |     v
    7 | |    } else {
    8 | |      ""
    9 | |   };
      | |___-
      |
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_EMPTY_STRING, @r#"
    fn main() {
      let x: Option<ByteArray> = Option::Some("Hello");
      // This is just a variable.
      x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_new_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_NEW, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = x {
    6 | |     v
    7 | |    } else {
    8 | |      ArrayTrait::new()
    9 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_NEW, @r"
    fn main() {
      let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = x {
    6 | |     v
    7 | |    } else {
    8 | |     0
    9 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    fn main() {
      let x: Option<u128> = Option::Some(1038);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_fixed_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = a {
    6 | |     v
    7 | |    } else {
    8 | |     [0; 5]
    9 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    fn main() {
      let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
      // This is just a variable.
      a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE, @r#"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = a {
    6 | |     v
    7 | |    } else {
    8 | |       ("", 0, false)
    9 | |   };
      | |___-
      |
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE, @r#"
    fn main() {
      let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
      // This is just a variable.
      a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ARRAY, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   if let Option::Some(v) = x {
    6 | |     v
    7 | |    } else {
    8 | |      array![]
    9 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_ARRAY, @r"
    fn main() {
      let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_comments_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_COMMENTS, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
      --> lib.cairo:5:3
       |
     5 | /   if let Option::Some(v) = a {
     6 | |     // testing with comments
    ...  |
    10 | |     Default::default()
    11 | |   };
       | |___-
       |
    ");
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_COMMENTS, @r#"
    fn main() {
      let a: Option<ByteArray> = Option::Some("Helok");
      // This is just a variable.
      a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_tuple_without_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE_WITHOUT_TRIGGER, @r#"
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_tuple_without_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_TUPLE_WITHOUT_TRIGGER, @r#"
    fn main() {
      let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
      // This is just a variable.
      if let Option::Some(v) = a {
        v
       } else {
          ("", 0, true)
      };
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_different_type_not_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#"
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#"
    fn main() {
      let a: Option<ByteArray> = Option::Some("Helok");
      // This is just a variable.
      if let Option::Some(_) = a {
        100
       } else {
        0
      };
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_without_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITHOUT_TRIGGER, @r#"
    "#);
}

#[test]
fn manual_unwrap_or_default_for_if_let_without_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_IF_LET_WITHOUT_TRIGGER, @r"
    fn main() {
      let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
      // This is just a variable.
      if let Option::Some(v) = a {
        v
       } else {
        [3; 5]
      };
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_tuple_without_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE_WITHOUT_TRIGGER, @r#"
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_tuple_without_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE_WITHOUT_TRIGGER, @r#"
    fn main() {
      let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));
      // This is just a variable.
      match x {
        Option::Some(v) => v,
        Option::None => ("sdkfh", 898, false)
      };
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => 0
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    fn main() {
      let x: Option<u128> = Option::Some(1038);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_EMPTY_STRING, @r#"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => ""
    8 | |   };
      | |___-
      |
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_EMPTY_STRING, @r#"
    fn main() {
      let x: Option<ByteArray> = Option::Some("Hello");
      // This is just a variable.
      x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_default_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DEFAULT, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match a {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => Default::default()
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DEFAULT, @r"
    fn main() {
      let a: Option<felt252> = Option::Some(1);
      // Somethings wrong.
      a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_new_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_NEW, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => ArrayTrait::new()
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_NEW, @r"
    fn main() {
      let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_fixed_array_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => [0; 5]
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    fn main() {
      let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE, @r#"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => ("", 0, false)
    8 | |   };
      | |___-
      |
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_TUPLE, @r#"
    fn main() {
      let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));
      // This is just a variable.
      x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ARRAY, @r"
    warning: Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3
      |
    5 | /   match x {
    6 | |     Option::Some(v) => v,
    7 | |     Option::None => array![]
    8 | |   };
      | |___-
      |
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_ARRAY, @r"
    fn main() {
      let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
      // This is just a variable.
      x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_comments_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_COMMENTS, @r#""#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_COMMENTS, @r"
    fn main() {
      let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
      // This is just a variable.
      match x {
        Option::Some(v) => {
          // Testing with comments
          v
        },
        Option::None => {
          // Testing with comments
          [0; 5]
        }
      };
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_different_type_not_trigger_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#""#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r"
    fn main() {
      let x: Option<u128> = Option::Some(1038);
      // This is just a variable.
      match x {
        Option::Some(_) => array![1, 2, 3, 4, 5],
        Option::None => array![]
      };
    }
    ");
}

#[test]
fn manual_unwrap_or_default_for_match_with_without_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_WITHOUT_TRIGGER, @r#""#);
}

#[test]
fn manual_unwrap_or_default_for_match_with_without_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_FOR_MATCH_WITH_WITHOUT_TRIGGER, @r"
    fn main() {
      let x: Option<u128> = Option::Some(1038);
      // This is just a variable.
      match x {
        Option::Some(v) => v,
        Option::None => 6778
      };
    }
    ");
}
