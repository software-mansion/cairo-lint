use crate::{test_lint_diagnostics, test_lint_fixer};

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DEFAULT: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DEFAULT: &str = r#"
fn main() {
  let a: Result<ByteArray, felt252> = Result::Ok("Helok");
  // This is just a variable.
  if let Result::Ok(v) = a {
    v
  } else {
    Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_EMPTY_STRING: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_EMPTY_STRING: &str = r#"
fn main() {
  let x: Result<ByteArray, felt252> = Result::Ok("Hello");
  // This is just a variable.
  if let Result::Ok(v) = x {
    v
  } else {
     ""
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_NEW: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_NEW: &str = r#"
fn main() {
  let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Result::Ok(v) = x {
    v
  } else {
     ArrayTrait::new()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ZERO_INTEGER: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ZERO_INTEGER: &str = r#"
fn main() {
  let x: Result<u128, felt252> = Result::Ok(1038);
  // This is just a variable.
  if let Result::Ok(v) = x {
    v
  } else {
    0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_FIXED_ARRAY: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_FIXED_ARRAY: &str = r#"
fn main() {
  let a: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Result::Ok(v) = a {
    v
  } else {
    [0; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_TUPLE: &str = r#"
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

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_TUPLE: &str = r#"
fn main() {
  let a: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
  // This is just a variable.
  if let Result::Ok(v) = a {
    v
  } else {
      ("", 0, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ARRAY: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  let _z = if let Option::Some(v) = x {
    v
  } else {
     array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ARRAY: &str = r#"
fn main() {
  let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  if let Result::Ok(v) = x {
    v
  } else {
     array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_COMMENTS: &str = r#"
fn main() {
  let a: Option<ByteArray> = Option::Some("Helok");
  // This is just a variable.
  if let Option::Some(v) = a {
    // testing with comments some arm
    v
  } else {
    // testing with comments none arm
    Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_COMMENTS: &str = r#"
fn main() {
  let a: Result<ByteArray, felt252> = Result::Ok("Helok");
  // This is just a variable.
  if let Result::Ok(v) = a {
    // testing with comments ok arm
    v
  } else {
    // testing with comments err arm 
    Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let a: Option<ByteArray> = Option::Some("Hello");
  // This is just a variable.
  if let Option::Some(_) = a {
    100
  } else {
    0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let a: Result<ByteArray, felt252> = Result::Ok("Hello");
  // This is just a variable.
  if let Result::Ok(_) = a {
    100
  } else {
    0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ZERO_INTEGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => 0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ZERO_INTEGER: &str = r#"
fn main() {
  let x: Result<u128, felt252> = Result::Ok(1038);
  // This is just a variable.
  match x {
    Result::Ok(v) => v,
    Result::Err(_) => 0
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_EMPTY_STRING: &str = r#"
fn main() {
  let x: Option<ByteArray> = Option::Some("Hello");
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ""
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_EMPTY_STRING: &str = r#"
fn main() {
  let x: Result<ByteArray, felt252> = Result::Ok("Hello");
  // This is just a variable.
  match x {
    Result::Ok(v) => v,
    Result::Err(_) => ""
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DEFAULT: &str = r#"
fn main() {
  let a: Option<felt252> = Option::Some(1);
  // Somethings wrong.
  match a {
    Option::Some(v) => v,
    Option::None => Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DEFAULT: &str = r#"
fn main() {
  let a: Result<felt252, felt252> = Result::Ok(1);
  // Somethings wrong.
  match a {
    Result::Ok(v) => v,
    Result::Err(_) => Default::default()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NEW: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ArrayTrait::new()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_NEW: &str = r#"
fn main() {
  let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Result::Ok(v) => v,
    Result::Err(_) => ArrayTrait::new()
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_FIXED_ARRAY: &str = r#"
fn main() {
  let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => [0; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_FIXED_ARRAY: &str = r#"
fn main() {
  let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Result::Ok(v) => v,
    Result::Err(_) => [0; 5]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_TUPLE: &str = r#"
fn main() {
  let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => ("", 0, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_TUPLE: &str = r#"
fn main() {
  let x: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
  // This is just a variable.
  let _z = match x {
    Result::Ok(v) => v,
    Result::Err(_) => ("", 0, false)
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ARRAY: &str = r#"
fn main() {
  let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => v,
    Option::None => array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ARRAY: &str = r#"
fn main() {
  let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Result::Ok(v) => v,
    Result::Err(_) => array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_COMMENTS: &str = r#"
fn main() {
  let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Option::Some(v) => {
      // Testing with comments
      v
    },
    Option::None => { // comment after { 
      // Testing with comments
      [0; 5]
      // comment before }
    }
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_COMMENTS: &str = r#"
fn main() {
  let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
  // This is just a variable.
  match x {
    Result::Ok(v) => {
      // Testing with comments ok arm
      v
    },
    Result::Err(_) => {
      // Testing with comments err arm
      [0; 5]
    }
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_SOME_BLOCK: &str = r#"
fn main() {
    let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
    // This is just a variable.
    match x {
        Option::Some(v) => { v },
        Option::None => [0; 5],
    };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_OK_BLOCK: &str = r#"
fn main() {
    let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
    // This is just a variable.
    match x {
        Result::Ok(v) => { v },
        Result::Err(_) => [0; 5],
    };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NONE_BLOCK: &str = r#"
fn main() {
    let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
    // This is just a variable.
    match x {
        Option::Some(v) => v,
        Option::None => { [0; 5] },
    };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ERR_BLOCK: &str = r#"
fn main() {
    let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
    // This is just a variable.
    match x {
        Result::Ok(v) => v,
        Result::Err(_) => { [0; 5] },
    };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let x: Option<u128> = Option::Some(1038);
  // This is just a variable.
  match x {
    Option::Some(_) => array![1, 2, 3, 4, 5],
    Option::None => array![]
  };
}
"#;

const MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER: &str = r#"
fn main() {
  let x: Result<u128, felt252> = Result::Ok(1038);
  // This is just a variable.
  match x {
    Result::Ok(_) => array![1, 2, 3, 4, 5],
    Result::Err(_) => array![]
  };
}
"#;

const IF_LET_WITH_COMMENT_OPTION: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(42);
    // Some before
    let _z = if let Option::Some(v) = a {
        v
    } else { // comment after { 
        // Some comment
        0 // Comment after value
        // comment before } 
    };
}
"#;

const MATCH_WITH_COMMENT_AFTER_ARROW: &str = r#"
fn main() {
    let a: Result<u64, felt252> = Result::Ok(54);
    // This is comment
    let _x = match a {
        Result::Ok(v) => v,
        Result::Err(_) => // comment after =>
            // Different comment 
            0 
    };
}
"#;

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_default_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DEFAULT, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DEFAULT, @r#"
    fn main() {
        let a: Option<ByteArray> = Option::Some("Helok");
        // This is just a variable.
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_default_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DEFAULT, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DEFAULT, @r#"
    fn main() {
        let a: Result<ByteArray, felt252> = Result::Ok("Helok");
        // This is just a variable.
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_EMPTY_STRING, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_EMPTY_STRING, @r#"
    fn main() {
        let x: Option<ByteArray> = Option::Some("Hello");
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_EMPTY_STRING, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_EMPTY_STRING, @r#"
    fn main() {
        let x: Result<ByteArray, felt252> = Result::Ok("Hello");
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_new_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_NEW, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_NEW, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_new_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_NEW, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_NEW, @r"
    fn main() {
        let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    fn main() {
        let x: Option<u128> = Option::Some(1038);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ZERO_INTEGER, @r"
    fn main() {
        let x: Result<u128, felt252> = Result::Ok(1038);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_fixed_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    fn main() {
        let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
        // This is just a variable.
        a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_fixed_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_FIXED_ARRAY, @r"
    fn main() {
        let a: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        // This is just a variable.
        a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_TUPLE, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_TUPLE, @r#"
    fn main() {
        let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
        // This is just a variable.
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_TUPLE, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_TUPLE, @r#"
    fn main() {
        let a: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
        // This is just a variable.
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:12-9:3
        let _z = if let Option::Some(v) = x {
     ____________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_ARRAY, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        let _z = x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-9:3
        if let Result::Ok(v) = x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_ARRAY, @r"
    fn main() {
        let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_comments_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_COMMENTS, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-11:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_COMMENTS, @r#"
    fn main() {
        let a: Option<ByteArray> = Option::Some("Helok");
        // This is just a variable.
        // testing with comments some arm
        // testing with comments none arm
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_comments_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_COMMENTS, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-11:3
        if let Result::Ok(v) = a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_COMMENTS, @r#"
    fn main() {
        let a: Result<ByteArray, felt252> = Result::Ok("Helok");
        // This is just a variable.
        // testing with comments ok arm
        // testing with comments err arm
        a.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_different_type_not_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#"
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#"
    fn main() {
        let a: Option<ByteArray> = Option::Some("Hello");
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
fn manual_unwrap_or_default_result_for_if_let_with_different_type_not_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#""#);
}

#[test]
fn manual_unwrap_or_default_result_for_if_let_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_IF_LET_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#"
    fn main() {
        let a: Result<ByteArray, felt252> = Result::Ok("Hello");
        // This is just a variable.
        if let Result::Ok(_) = a {
            100
        } else {
            0
        };
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    fn main() {
        let x: Option<u128> = Option::Some(1038);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_zero_integer_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_zero_integer_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ZERO_INTEGER, @r"
    fn main() {
        let x: Result<u128, felt252> = Result::Ok(1038);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_EMPTY_STRING, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_EMPTY_STRING, @r#"
    fn main() {
        let x: Option<ByteArray> = Option::Some("Hello");
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_empty_string_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_EMPTY_STRING, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_empty_string_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_EMPTY_STRING, @r#"
    fn main() {
        let x: Result<ByteArray, felt252> = Result::Ok("Hello");
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_default_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DEFAULT, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DEFAULT, @r"
    fn main() {
        let a: Option<felt252> = Option::Some(1);
        // Somethings wrong.
        a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_default_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DEFAULT, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match a {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_default_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DEFAULT, @r"
    fn main() {
        let a: Result<felt252, felt252> = Result::Ok(1);
        // Somethings wrong.
        a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_new_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NEW, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NEW, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_new_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_NEW, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_new_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_NEW, @r"
    fn main() {
        let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_fixed_array_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    fn main() {
        let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_fixed_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_fixed_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_FIXED_ARRAY, @r"
    fn main() {
        let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_TUPLE, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_TUPLE, @r#"
    fn main() {
        let x: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_tuple_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_TUPLE, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:12-8:3
        let _z = match x {
     ____________^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_tuple_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_TUPLE, @r#"
    fn main() {
        let x: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
        // This is just a variable.
        let _z = x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ARRAY, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_array_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ARRAY, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_array_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_ARRAY, @r"
    fn main() {
        let x: Result<Array<u128>, felt252> = Result::Ok(array![1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_comments_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_COMMENTS, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-15:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_COMMENTS, @r"
    fn main() {
        let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
        // This is just a variable.
        // Testing with comments
        // comment after {
        // Testing with comments
        // comment before }
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_comments_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_COMMENTS, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:3-14:3
        match x {
     ___^
    | ...
    |   };
    |___^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_comments_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_COMMENTS, @r"
    fn main() {
        let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        // This is just a variable.
        // Testing with comments ok arm
        // Testing with comments err arm
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_some_block_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_SOME_BLOCK, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:5-8:5
          match x {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_some_block_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_SOME_BLOCK, @r#"
    fn main() {
        let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_ok_block_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_OK_BLOCK, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:5-8:5
          match x {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_ok_block_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_OK_BLOCK, @r#"
    fn main() {
        let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    "#);
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_none_block_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NONE_BLOCK, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:5-8:5
          match x {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_none_block_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_NONE_BLOCK, @r"
    fn main() {
        let x: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_err_block_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ERR_BLOCK, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:5-8:5
          match x {
     _____^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_err_block_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_ERR_BLOCK, @r"
    fn main() {
        let x: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        // This is just a variable.
        x.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_different_type_not_trigger_diagnostic() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#""#);
}

#[test]
fn manual_unwrap_or_default_option_for_match_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_OPTION_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r"
    fn main() {
        let x: Option<u128> = Option::Some(1038);
        // This is just a variable.
        match x {
            Option::Some(_) => array![1, 2, 3, 4, 5],
            Option::None => array![],
        };
    }
    ");
}
#[test]
fn manual_unwrap_or_default_result_for_match_with_different_type_not_trigger_diagnostics() {
    test_lint_diagnostics!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r#""#);
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_different_type_not_trigger_fixer() {
    test_lint_fixer!(MANUAL_UNWRAP_OR_DEFAULT_RESULT_FOR_MATCH_WITH_DIFFERENT_TYPE_NOT_TRIGGER, @r"
    fn main() {
        let x: Result<u128, felt252> = Result::Ok(1038);
        // This is just a variable.
        match x {
            Result::Ok(_) => array![1, 2, 3, 4, 5],
            Result::Err(_) => array![],
        };
    }
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_comment_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_COMMENT_OPTION, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:14-11:5
          let _z = if let Option::Some(v) = a {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_option_for_if_let_with_comment_fixer() {
    test_lint_fixer!(IF_LET_WITH_COMMENT_OPTION, @r"
    fn main() {
        let a: Option<u128> = Option::Some(42);
        // Some before
        // comment after {
        // Some comment
        // Comment after value
        // comment before }
        let _z = a.unwrap_or_default();
    }
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_comment_after_arrow_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_COMMENT_AFTER_ARROW, @r"
    Plugin diagnostic: This can be done in one call with `.unwrap_or_default()`
     --> lib.cairo:5:14-10:5
          let _x = match a {
     ______________^
    | ...
    |     };
    |_____^
    ");
}

#[test]
fn manual_unwrap_or_default_result_for_match_with_comment_after_arrow_fixer() {
    test_lint_fixer!(MATCH_WITH_COMMENT_AFTER_ARROW, @r"
    fn main() {
        let a: Result<u64, felt252> = Result::Ok(54);
        // This is comment
        // comment after =>
        // Different comment
        let _x = a.unwrap_or_default();
    }
    ");
}
