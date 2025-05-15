use crate::{test_lint_diagnostics, test_lint_fixer};

const IF_LET_WITH_CONSTANT_OPTION: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(42);

    let _x = if let Option::Some(v) = a {
        v
    } else {
        777
    };
}
"#;

const IF_LET_WITH_STRING_LITERAL_OPTION: &str = r#"
fn main() {
    let a: Option<ByteArray> = Option::Some("Hi");

    if let Option::Some(v) = a {
        v
    } else {
        "backup"
    };
}
"#;

const IF_LET_WITH_ARRAY_LITERAL_OPTION: &str = r#"
fn main() {
  let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);

  if let Option::Some(v) = a {
    v
   } else {
    [3; 5]
  };
}
"#;

const IF_LET_WITH_ARRAY_MACRO_OPTION: &str = r#"
fn main() {
    let x: Option<Array<u128>> = Option::Some(array![2, 2]);

    if let Option::Some(v) = x {
        v
    } else {
        array![9, 9, 9]
    };
}
"#;

const MATCH_WITH_CONSTANT_OPTION: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(51);
    let _x = match a {
        Option::Some(v) => {
            v
        },
        Option::None => 777
    };
}
"#;

const MATCH_WITH_STRING_LITERAL_OPTION: &str = r#"
fn main() {
    let s: Option<ByteArray> = Option::Some("Hello");
    match s {
        Option::Some(v) => v,
        Option::None => "manual fallback"
    };
}
"#;

const IF_LET_WITH_TUPLE_OPTION: &str = r#"
fn main() {
  let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
  if let Option::Some(v) = a {
    v
   } else {
      ("", 0, true)
  };
}
"#;

const MATCH_WITH_ARRAY_MACRO_OPTION: &str = r#"
fn main() {
    let arr: Option<Array<u128>> = Option::Some(array![11, 22]);
    let _x = match arr {
        Option::Some(v) => v,
        Option::None => array![100, 200]
    };
}
"#;

const MATCH_WITH_TUPLE_OPTION: &str = r#"
fn main() {
  let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));

  match x {
    Option::Some(v) => v,
    Option::None => ("sdkfh", 898, false)
  };
}
"#;

const IF_LET_WITH_CONSTANT_RESULT: &str = r#"
fn main() {
    let a: Result<u128, felt252> = Result::Ok(42);

    let _x = if let Result::Ok(v) = a {
        v
    } else {
        777
    };
}
"#;

const IF_LET_WITH_STRING_LITERAL_RESULT: &str = r#"
fn main() {
    let a: Result<ByteArray, felt252> = Result::Ok("Hi");

    if let Result::Ok(v) = a {
        v
    } else {
        "backup"
    };
}
"#;

const MATCH_WITH_CONSTANT_RESULT: &str = r#"
fn main() {
    let a: Result<u128, felt252> = Result::Ok(51);
    match a {
        Result::Ok(v) => {
            v
        },
        Result::Err(_) => 777
    };
}
"#;

const MATCH_WITH_STRING_LITERAL_RESULT: &str = r#"
fn main() {
    let s: Result<ByteArray, felt252> = Result::Ok("Hello");
    match s {
        Result::Ok(v) => v,
        Result::Err(_) => "manual fallback"
    };
}
"#;

const IF_LET_WITH_ARRAY_LITERAL_RESULT: &str = r#"
fn main() {
    let a: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
    if let Result::Ok(v) = a {
        v
    } else {
        [3; 5]
    };
}
"#;

const IF_LET_WITH_TUPLE_RESULT: &str = r#"
fn main() {
    let a: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
    if let Result::Ok(v) = a {
        v
    } else {
        ("", 0, true)
    };
}
"#;

const MATCH_WITH_FIXED_ARRAY_RESULT: &str = r#"
fn main() {
    let arr: Result<[u8; 4], felt252> = Result::Ok([1, 2, 3, 4]);
    match arr {
        Result::Ok(v) => v,
        Result::Err(_) => [5, 5, 5, 5]
    };
}
"#;

const MATCH_WITH_ARRAY_MACRO_RESULT: &str = r#"
fn main() {
    let arr: Result<Array<u128>, felt252> = Result::Ok(array![11, 22]);
    match arr {
        Result::Ok(v) => v,
        Result::Err(_) => {
            array![100, 200]
        }
    };
}
"#;

const MATCH_WITH_NESTED_IF_RESULT: &str = r#"
fn main() {
    let a: Result<u128, felt252> = Result::Ok(99);
    match a {
        Result::Ok(v) => v,
        Result::Err(_) => {
            if true {
                1
            } else {
                2
            }
        }
    };
}
"#;

const IF_LET_WITH_MATCH_IN_ELSE_OPTION: &str = r#"
fn main() {
    let x: Option<u128> = Option::Some(123);
    if let Option::Some(v) = x {
        v
    } else {
        // comment
        match true {
            true => 10,
            false => 20,
        }

        // comment
    //third comment

    };
}
"#;

const IF_LET_WITH_IF_IN_ELSE_OPTION: &str = r#"
fn main() {
    let x: Option<u128> = Option::Some(123);

    #[allow(collapsible_if_else)]
    if let Option::Some(v) = x {
        v
    } else { 
        if true {
            10
        } else {
            20
        }
    };
}
"#;

const MATCH_WITH_OPTION_WITH_COMMENT: &str = r#"
fn main() {
    let a: Option<[u64; 2]> = Option::Some([10, 20]);

    let _x = match a {
        Option::Some(v) => {
            v
        },
        Option::None => {
            [1, 2]
            // comment
        }
    };
}
"#;

const MATCH_WITH_RESULT_WITH_COMMENT: &str = r#"
fn main() {
    let a: Result<[u64; 2], felt252> = Result::Ok([10, 20]);
    let _x = match a {
        Result::Ok(v) => {
            v
        },
        Result::Err(_) => 
            // comment
            [1, 2]
    };
}
"#;

const IF_LET_WITH_RESULT_WITH_COMMENT: &str = r#"
fn main() {
    let x: Result<Array<u128>, felt252> = Result::Ok(array![2, 2]);
    if let Result::Ok(v) = x {
        v
    } else {
        // comment
        array![9, 9, 9]
    };
}
"#;

const IF_LET_WITH_COMMENT_OPTION: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(42);

    if let Option::Some(v) = a {
        v
    } else {
        777
        // comment
    };
}
"#;

const MATCH_WITH_COMMENT_AFTER_BRACE: &str = r#"
fn main() {
    let a: Option<u64> = Option::Some(42);
    let _x = match a {
        Option::Some(v) => v,
        Option::None => { // comment after {
            123
        }
    };
}
"#;

const MATCH_WITH_COMMENT_AFTER_ARROW: &str = r#"
fn main() {
    let a: Result<u64, felt252> = Result::Ok(54);
    let _x = match a {
        Result::Ok(v) => v,
        Result::Err(_) => // comment after =>
            231
    };
}
"#;

const IF_LET_WITH_COMMENT_AFTER_BRACE: &str = r#"
fn main() {
    let x: Option<Array<u128>> = Option::Some(array![1, 2, 3]);
    if let Option::Some(v) = x {
        v
    } else { // comment after {
        array![1, 2, 3]
    };
}
"#;

const MATCH_WITH_RESULT_OK_COMMENT: &str = r#"
fn main() {
    let a: Result<[u64; 2], felt252> = Result::Ok([10, 20]);
    match a {
        Result::Ok(v) => {
            // this is ok arm
            v
        },
        Result::Err(_) => [1, 2]
    };
}
"#;

const IF_LET_WITH_OPTION_SOME_COMMENT: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(42);

    if let Option::Some(v) = a {
        // some comment
        v
    } else {
        777
    };
}
"#;

const MATCH_WITH_MORE_STATEMENTS_OPTION: &str = r#"
fn main() {
    let a: Option<[u64; 2]> = Option::Some([10, 20]);

    match a {
        Option::Some(v) => {
            v
        },
        Option::None => {
            println!("Hello World");
            [1, 2]
        }
    };
}
"#;

const MATCH_WITH_MORE_STATEMENTS_RESULT: &str = r#"
fn main() {
    let a: Result<[u64; 2], felt252> = Result::Ok([10, 20]);
    match a {
        Result::Ok(v) => {
            v
        },
        Result::Err(_) => {
            println!("Hello World");
            [1, 2]
        }
    };
}
"#;

const IF_LET_WITH_MORE_STATEMENTS_RESULT: &str = r#"
fn main() {
    let x: Result<Array<u128>, felt252> = Result::Ok(array![2, 2]);
    if let Result::Ok(v) = x {
        v
    } else {
        println!("Hello World");
        array![9, 9, 9]
    };
}
"#;

const IF_LET_WITH_MORE_STATEMENTS_OPTION: &str = r#"
fn main() {
    let a: Option<u128> = Option::Some(42);

    if let Option::Some(v) = a {
        v
    } else {
        println!("Hello World");
        777
    };
}
"#;

const ALLOW_MATCH_WITH_TUPLE_OPTION: &str = r#"
fn main() {
  let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));

  #[allow(manual_unwrap_or)]
  match x {
    Option::Some(v) => v,
    Option::None => ("sdkfh", 898, false)
  };
}
"#;

const ALLOW_IF_LET_WITH_ARRAY_MACRO_RESULT: &str = r#"
fn main() {
    let x: Result<Array<u128>, felt252> = Result::Ok(array![2, 2]);

    #[allow(manual_unwrap_or)]
    if let Result::Ok(v) = x {
        v
    } else {
        array![9, 9, 9]
    };
}
"#;

#[test]
fn if_let_with_constant_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_CONSTANT_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:14-9:5
          let _x = if let Option::Some(v) = a {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_constant_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_CONSTANT_OPTION, @r"
    fn main() {
        let a: Option<u128> = Option::Some(42);

        let _x = a.unwrap_or(777);
    }
    ");
}

#[test]
fn if_let_with_string_literal_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_STRING_LITERAL_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:5-9:5
          if let Option::Some(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_string_literal_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_STRING_LITERAL_OPTION, @r#"
    fn main() {
        let a: Option<ByteArray> = Option::Some("Hi");

        a.unwrap_or("backup");
    }
    "#);
}

#[test]
fn if_let_with_array_literal_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_ARRAY_LITERAL_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:3-9:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    "#);
}

#[test]
fn if_let_with_array_literal_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_ARRAY_LITERAL_OPTION, @r"
    fn main() {
      let a: Option<[u64; 5]> = Option::Some([1, 2, 3, 4, 5]);

      a.unwrap_or([3; 5]);
    }
    ");
}

#[test]
fn if_let_with_array_macro_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_ARRAY_MACRO_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:5-9:5
          if let Option::Some(v) = x {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_array_macro_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_ARRAY_MACRO_OPTION, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![2, 2]);

        x.unwrap_or(array![9, 9, 9]);
    }
    ");
}

#[test]
fn match_with_constant_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_CONSTANT_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:14-9:5
          let _x = match a {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_constant_option_fixer() {
    test_lint_fixer!(MATCH_WITH_CONSTANT_OPTION, @r"
    fn main() {
        let a: Option<u128> = Option::Some(51);
        let _x = a.unwrap_or(777);
    }
    ");
}

#[test]
fn match_with_string_literal_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_STRING_LITERAL_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-7:5
          match s {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_string_literal_option_fixer() {
    test_lint_fixer!(MATCH_WITH_STRING_LITERAL_OPTION, @r#"
    fn main() {
        let s: Option<ByteArray> = Option::Some("Hello");
        s.unwrap_or("manual fallback");
    }
    "#);
}

#[test]
fn if_let_with_tuple_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_TUPLE_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:3-8:3
        if let Option::Some(v) = a {
     ___^
    | ...
    |   };
    |___^
    "#);
}

#[test]
fn if_let_with_tuple_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_TUPLE_OPTION, @r#"
    fn main() {
      let a: Option<(ByteArray, u128, bool)> = Option::Some(("James", 90, true));
      a.unwrap_or(("", 0, true));
    }
    "#);
}

#[test]
fn match_with_array_macro_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_ARRAY_MACRO_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:14-7:5
          let _x = match arr {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_array_macro_option_fixer() {
    test_lint_fixer!(MATCH_WITH_ARRAY_MACRO_OPTION, @r"
    fn main() {
        let arr: Option<Array<u128>> = Option::Some(array![11, 22]);
        let _x = arr.unwrap_or(array![100, 200]);
    }
    ");
}

#[test]
fn match_with_tuple_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_TUPLE_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:3-8:3
        match x {
     ___^
    | ...
    |   };
    |___^
    "#);
}

#[test]
fn match_with_tuple_option_fixer() {
    test_lint_fixer!(MATCH_WITH_TUPLE_OPTION, @r#"
    fn main() {
      let x: Option<(ByteArray, u128, bool)> =Option::Some(("James", 90, true));

      x.unwrap_or(("sdkfh", 898, false));
    }
    "#);
}

#[test]
fn if_let_with_constant_result_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_CONSTANT_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:14-9:5
          let _x = if let Result::Ok(v) = a {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_constant_result_fixer() {
    test_lint_fixer!(IF_LET_WITH_CONSTANT_RESULT, @r"
    fn main() {
        let a: Result<u128, felt252> = Result::Ok(42);

        let _x = a.unwrap_or(777);
    }
    ");
}

#[test]
fn if_let_with_string_literal_result_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_STRING_LITERAL_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:5-9:5
          if let Result::Ok(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_string_literal_result_fixer() {
    test_lint_fixer!(IF_LET_WITH_STRING_LITERAL_RESULT, @r#"
    fn main() {
        let a: Result<ByteArray, felt252> = Result::Ok("Hi");

        a.unwrap_or("backup");
    }
    "#);
}

#[test]
fn match_with_constant_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_CONSTANT_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-9:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_constant_result_fixer() {
    test_lint_fixer!(MATCH_WITH_CONSTANT_RESULT, @r"
    fn main() {
        let a: Result<u128, felt252> = Result::Ok(51);
        a.unwrap_or(777);
    }
    ");
}

#[test]
fn match_with_string_literal_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_STRING_LITERAL_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-7:5
          match s {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_string_literal_result_fixer() {
    test_lint_fixer!(MATCH_WITH_STRING_LITERAL_RESULT, @r#"
    fn main() {
        let s: Result<ByteArray, felt252> = Result::Ok("Hello");
        s.unwrap_or("manual fallback");
    }
    "#);
}

#[test]
fn if_let_with_array_literal_result_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_ARRAY_LITERAL_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-8:5
          if let Result::Ok(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_array_literal_result_fixer() {
    test_lint_fixer!(IF_LET_WITH_ARRAY_LITERAL_RESULT, @r"
    fn main() {
        let a: Result<[u64; 5], felt252> = Result::Ok([1, 2, 3, 4, 5]);
        a.unwrap_or([3; 5]);
    }
    ");
}

#[test]
fn if_let_with_tuple_result_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_TUPLE_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-8:5
          if let Result::Ok(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_tuple_result_fixer() {
    test_lint_fixer!(IF_LET_WITH_TUPLE_RESULT, @r#"
    fn main() {
        let a: Result<(ByteArray, u128, bool), felt252> = Result::Ok(("James", 90, true));
        a.unwrap_or(("", 0, true));
    }
    "#);
}

#[test]
fn match_with_fixed_array_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_FIXED_ARRAY_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-7:5
          match arr {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_fixed_array_result_fixer() {
    test_lint_fixer!(MATCH_WITH_FIXED_ARRAY_RESULT, @r"
    fn main() {
        let arr: Result<[u8; 4], felt252> = Result::Ok([1, 2, 3, 4]);
        arr.unwrap_or([5, 5, 5, 5]);
    }
    ");
}

#[test]
fn match_with_array_macro_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_ARRAY_MACRO_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-9:5
          match arr {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_array_macro_result_fixer() {
    test_lint_fixer!(MATCH_WITH_ARRAY_MACRO_RESULT, @r"
    fn main() {
        let arr: Result<Array<u128>, felt252> = Result::Ok(array![11, 22]);
        arr.unwrap_or(array![100, 200]);
    }
    ");
}

#[test]
fn match_with_nested_if_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_NESTED_IF_RESULT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-13:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_nested_if_result_fixer() {
    test_lint_fixer!(MATCH_WITH_NESTED_IF_RESULT, @r"
    fn main() {
        let a: Result<u128, felt252> = Result::Ok(99);
        a.unwrap_or({
            if true {
                1
            } else {
                2
            }
        });
    }
    ");
}

#[test]
fn if_let_with_match_in_else_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_MATCH_IN_ELSE_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-16:5
          if let Option::Some(v) = x {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_match_in_else_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_MATCH_IN_ELSE_OPTION, @r"
    fn main() {
        let x: Option<u128> = Option::Some(123);
        x.unwrap_or({
            // comment
            match true {
                true => 10,
                false => 20,
            }

            // comment
        //third comment

        });
    }
    ");
}

#[test]
fn if_let_with_if_in_else_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_IF_IN_ELSE_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:6:5-14:5
          if let Option::Some(v) = x {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_if_in_else_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_IF_IN_ELSE_OPTION, @r"
    fn main() {
        let x: Option<u128> = Option::Some(123);

        #[allow(collapsible_if_else)]
        x.unwrap_or({ 
            if true {
                10
            } else {
                20
            }
        });
    }
    ");
}

#[test]
fn match_with_option_with_comment_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_OPTION_WITH_COMMENT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:14-13:5
          let _x = match a {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_option_with_comment_fixer() {
    test_lint_fixer!(MATCH_WITH_OPTION_WITH_COMMENT, @r"
    fn main() {
        let a: Option<[u64; 2]> = Option::Some([10, 20]);

        let _x = a.unwrap_or({
            [1, 2]
            // comment
        });
    }
    ");
}

#[test]
fn match_with_result_with_comment_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_RESULT_WITH_COMMENT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:14-11:5
          let _x = match a {
     ______________^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_result_with_comment_fixer() {
    test_lint_fixer!(MATCH_WITH_RESULT_WITH_COMMENT, @r"
    fn main() {
        let a: Result<[u64; 2], felt252> = Result::Ok([10, 20]);
        let _x = a.unwrap_or(
            // comment
            [1, 2]
        );
    }
    ");
}

#[test]
fn if_let_with_result_with_comment_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_RESULT_WITH_COMMENT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-9:5
          if let Result::Ok(v) = x {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_result_with_comment_fixer() {
    test_lint_fixer!(IF_LET_WITH_RESULT_WITH_COMMENT, @r"
    fn main() {
        let x: Result<Array<u128>, felt252> = Result::Ok(array![2, 2]);
        x.unwrap_or({
            // comment
            array![9, 9, 9]
        });
    }
    ");
}

#[test]
fn if_let_with_comment_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_COMMENT_OPTION, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:5-10:5
          if let Option::Some(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_comment_option_fixer() {
    test_lint_fixer!(IF_LET_WITH_COMMENT_OPTION, @r"
    fn main() {
        let a: Option<u128> = Option::Some(42);

        a.unwrap_or({
            777
            // comment
        });
    }
    ");
}

#[test]
fn match_with_result_ok_comment_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_RESULT_OK_COMMENT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:4:5-10:5
          match a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn match_with_result_ok_comment_fixer() {
    test_lint_fixer!(MATCH_WITH_RESULT_OK_COMMENT, @r"
    fn main() {
        let a: Result<[u64; 2], felt252> = Result::Ok([10, 20]);
        a.unwrap_or([1, 2]);
    }
    ");
}

#[test]
fn if_let_with_option_some_comment_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_OPTION_SOME_COMMENT, @r#"
    Plugin diagnostic: Manual `unwrap_or` detected. Consider using `unwrap_or()` instead.
     --> lib.cairo:5:5-10:5
          if let Option::Some(v) = a {
     _____^
    | ...
    |     };
    |_____^
    "#);
}

#[test]
fn if_let_with_option_some_comment_fixer() {
    test_lint_fixer!(IF_LET_WITH_OPTION_SOME_COMMENT, @r"
    fn main() {
        let a: Option<u128> = Option::Some(42);

        a.unwrap_or(777);
    }
    ");
}

#[test]
fn match_with_comment_after_brace_fixer() {
    test_lint_fixer!(MATCH_WITH_COMMENT_AFTER_BRACE, @r"
    fn main() {
        let a: Option<u64> = Option::Some(42);
        let _x = a.unwrap_or({ // comment after {
            123
        });
    }
    ");
}

#[test]
fn match_with_comment_after_arrow_fixer() {
    test_lint_fixer!(MATCH_WITH_COMMENT_AFTER_ARROW, @r"
    fn main() {
        let a: Result<u64, felt252> = Result::Ok(54);
        let _x = a.unwrap_or( // comment after =>
            231
        );
    }
    ");
}

#[test]
fn if_let_with_comment_after_brace_fixer() {
    test_lint_fixer!(IF_LET_WITH_COMMENT_AFTER_BRACE, @r"
    fn main() {
        let x: Option<Array<u128>> = Option::Some(array![1, 2, 3]);
        x.unwrap_or({ // comment after {
            array![1, 2, 3]
        });
    }
    ");
}

#[test]
fn match_with_more_statements_option_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_MORE_STATEMENTS_OPTION, @r"");
}

#[test]
fn match_with_more_statements_result_diagnostics() {
    test_lint_diagnostics!(MATCH_WITH_MORE_STATEMENTS_RESULT, @r"");
}

#[test]
fn if_let_with_more_statements_result_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_MORE_STATEMENTS_RESULT, @r"");
}

#[test]
fn if_let_with_more_statements_option_diagnostics() {
    test_lint_diagnostics!(IF_LET_WITH_MORE_STATEMENTS_OPTION, @r"");
}

#[test]
fn allow_match_with_tuple_option_diagnostics() {
    test_lint_diagnostics!(ALLOW_MATCH_WITH_TUPLE_OPTION, @r"");
}

#[test]
fn allow_if_let_with_array_macro_result_diagnostics() {
    test_lint_diagnostics!(ALLOW_IF_LET_WITH_ARRAY_MACRO_RESULT, @r"");
}
