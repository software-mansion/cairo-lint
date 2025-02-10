use crate::{test_lint_diagnostics, test_lint_fixer};

const SIMPLE_LOOP_MATCH_POP_FRONT: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        match a.pop_front() {
            Option::Some(val) => println!("{val}"),
            Option::None => { break; },
        }
    }
}
"#;

const SIMPLE_LOOP_MATCH_POP_FRONT_WITH_LET: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        let _b = a;
        match a.pop_front() {
            Option::Some(val) => println!("{val}"),
            Option::None => { break; },
        }
    }
}
"#;

const SIMPLE_LOOP_MATCH_POP_FRONT_IMPL_PATH: &str = r#"
use core::array::SpanImpl;
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        match SpanImpl::pop_front(ref a) {
            Option::Some(val) => println!("{val}"),
            Option::None => { break; },
        }
    }
}
"#;

const SIMPLE_LOOP_MATCH_POP_FRONT_MULTIPLE_DOTS: &str = r#"
struct A {
    b: B
}
struct B {
    c: Span<u32>
}
fn main() {
    let mut a = A { b: B {c: array![1, 2, 3, 4, 5].span()} };
    loop {
        match a.b.c.pop_front() {
            Option::Some(val) => println!("{val}"),
            Option::None => { break; },
        }
    }
}
"#;

const LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        match a.pop_front() {
            Option::Some(val) => {
                // This is a comment
                println!("{val}")
            },
            Option::None => { 
                break;
            },
        }
    }
}
"#;

const LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME_ALLOWED: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    #[allow(loop_match_pop_front)]
    loop {
        match a.pop_front() {
            Option::Some(val) => {
                // This is a comment
                println!("{val}")
            },
            Option::None => { 
                break;
            },
        }
    }
}
"#;

const LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_NONE: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        match a.pop_front() {
            Option::Some(val) => println!("{val}"),
            Option::None => { 
                // This is a comment
                break;
            },
        }
    }
}
"#;

const LOOP_MATCH_POP_FRONT_WITH_STUFF_IN_NONE: &str = r#"
fn main() {
    let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
    loop {
        match a.pop_front() {
            Option::Some(val) => println!("{val}"),
            Option::None => { 
                println!("Finished looping");
                break;
            },
        }
    }
}
"#;

#[test]
fn simple_loop_match_pop_front_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_MATCH_POP_FRONT, @r"
    warning: Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`
     --> lib.cairo:4:5
      |
    4 | /     loop {
    5 | |         match a.pop_front() {
    ... |
    8 | |         }
    9 | |     }
      | |_____-
      |
    ");
}

#[test]
fn simple_loop_match_pop_front_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_MATCH_POP_FRONT, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();

        for val in a {
            println!("{val}")
        };
    }
    "#);
}

#[test]
fn simple_loop_match_pop_front_with_let_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_MATCH_POP_FRONT_WITH_LET, @r#"
    "#);
}

#[test]
fn simple_loop_match_pop_front_with_let_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_MATCH_POP_FRONT_WITH_LET, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
        loop {
            let _b = a;
            match a.pop_front() {
                Option::Some(val) => println!("{val}"),
                Option::None => { break; },
            }
        }
    }
    "#);
}

#[test]
fn simple_loop_match_pop_front_impl_path_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_MATCH_POP_FRONT_IMPL_PATH, @r"
    warning: Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`
      --> lib.cairo:5:5
       |
     5 | /     loop {
     6 | |         match SpanImpl::pop_front(ref a) {
    ...  |
     9 | |         }
    10 | |     }
       | |_____-
       |
    ");
}

#[test]
fn simple_loop_match_pop_front_impl_path_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_MATCH_POP_FRONT_IMPL_PATH, @r#"
    use core::array::SpanImpl;
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();

        for val in a {
            println!("{val}")
        };
    }
    "#);
}

#[test]
fn simple_loop_match_pop_front_multiple_dots_diagnostics() {
    test_lint_diagnostics!(SIMPLE_LOOP_MATCH_POP_FRONT_MULTIPLE_DOTS, @r"
    warning: Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`
      --> lib.cairo:10:5
       |
    10 | /     loop {
    11 | |         match a.b.c.pop_front() {
    ...  |
    14 | |         }
    15 | |     }
       | |_____-
       |
    ");
}

#[test]
fn simple_loop_match_pop_front_multiple_dots_fixer() {
    test_lint_fixer!(SIMPLE_LOOP_MATCH_POP_FRONT_MULTIPLE_DOTS, @r#"
    struct A {
        b: B
    }
    struct B {
        c: Span<u32>
    }
    fn main() {
        let mut a = A { b: B {c: array![1, 2, 3, 4, 5].span()} };

        for val in a.b.c {
            println!("{val}")
        };
    }
    "#);
}

#[test]
fn loop_match_pop_front_with_comment_in_some_diagnostics() {
    test_lint_diagnostics!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME, @r"
    warning: Plugin diagnostic: you seem to be trying to use `loop` for iterating over a span. Consider using `for in`
      --> lib.cairo:4:5
       |
     4 | /     loop {
     5 | |         match a.pop_front() {
    ...  |
    13 | |         }
    14 | |     }
       | |_____-
       |
    ");
}

#[test]
fn loop_match_pop_front_with_comment_in_some_fixer() {
    test_lint_fixer!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();

        for val in a {
            // This is a comment
            println!("{val}")

        };
    }
    "#);
}

#[test]
fn loop_match_pop_front_with_comment_in_some_allowed_diagnostics() {
    test_lint_diagnostics!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME_ALLOWED, @r#"
    "#);
}

#[test]
fn loop_match_pop_front_with_comment_in_some_allowed_fixer() {
    test_lint_fixer!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_SOME_ALLOWED, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
        #[allow(loop_match_pop_front)]
        loop {
            match a.pop_front() {
                Option::Some(val) => {
                    // This is a comment
                    println!("{val}")
                },
                Option::None => { 
                    break;
                },
            }
        }
    }
    "#);
}

#[test]
fn loop_match_pop_front_with_comment_in_none_diagnostics() {
    test_lint_diagnostics!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_NONE, @r#"
    "#);
}

#[test]
fn loop_match_pop_front_with_comment_in_none_fixer() {
    test_lint_fixer!(LOOP_MATCH_POP_FRONT_WITH_COMMENT_IN_NONE, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
        loop {
            match a.pop_front() {
                Option::Some(val) => println!("{val}"),
                Option::None => { 
                    // This is a comment
                    break;
                },
            }
        }
    }
    "#);
}

#[test]
fn loop_match_pop_front_with_stuff_in_none_diagnostics() {
    test_lint_diagnostics!(LOOP_MATCH_POP_FRONT_WITH_STUFF_IN_NONE, @r#"
    "#);
}

#[test]
fn loop_match_pop_front_with_stuff_in_none_fixer() {
    test_lint_fixer!(LOOP_MATCH_POP_FRONT_WITH_STUFF_IN_NONE, @r#"
    fn main() {
        let mut a: Span<u32> = array![1, 2, 3, 4, 5].span();
        loop {
            match a.pop_front() {
                Option::Some(val) => println!("{val}"),
                Option::None => { 
                    println!("Finished looping");
                    break;
                },
            }
        }
    }
    "#);
}
