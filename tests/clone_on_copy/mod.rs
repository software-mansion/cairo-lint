use crate::{test_lint_diagnostics, test_lint_fixer};

const CLONE_NUMERIC_TYPE: &str = r#"
fn main() {
    let a: u32 = 42;
    let b = a.clone();
    println!("{}", b);
}
"#;

const CLONE_FELT252: &str = r#"
fn main() {
    let a: felt252 = 'hello'
    let b = a.clone();
    println!("{}", b);
}
"#;

const CLONE_STRUCT: &str = r#"
#[derive(Copy, Drop)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    println!("{}, {}", p1.x, p2.y);
}
"#;

const CLONE_NON_COPY_STRUCT: &str = r#"
#[derive(Clone, Drop)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.clone();
    println!("{}, {}", p1.x, p2.y);
}
"#;

const CLONE_TUPLE: &str = r#"
fn main() {
    let t: (u32, felt252) = (42, 'hello');
    let t_clone = t.clone();
    println!("{:?}", t_clone);
}
"#;

const CLONE_ARRAY: &str = r#"
fn main() {
    let arr: [u32; 3] = [1, 2, 3];
    let arr_clone = arr.clone();
    println!("{:?}", arr_clone);
}
"#;

const CLONE_IN_IMPL: &str = r#"
#[derive(Copy, Drop)]
struct Point {
    x: u32,
    y: u32,
}

trait TMovable {
    fn move(self: @Point, dx: u32, dy: u32) -> Point;
}

impl Movable of TMovable {
    fn move(self: @Point, dx: u32, dy: u32) -> Point {
        let new_point = self.clone();
        Point { x: new_point.x + dx, y: new_point.y + dy }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.move(5, 5);
    println!("{}, {}", p1.x, p2.y);
}
"#;

const CLONE_ON_FUNCTION: &str = r#"
fn some_function() -> u32 {
    42
}

fn main() {
    let b = some_function().clone();
    println!("{}", b);
}
"#;

const CLONE_WITH_SNAPSHOT: &str = r#"
#[derive(Copy, Drop)]
struct Point {
    x: u32,
    y: u32,
}

fn duplicate(point: @Point) -> Point {
    let cloned_point = point.clone();
    cloned_point
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = duplicate(@p1);

    println!("Original: ({}, {}), Cloned: ({}, {})", p1.x, p1.y, p2.x, p2.y);
}
"#;

const ALLOW_CLONE_IN_IMPL: &str = r#"
#[derive(Copy, Drop)]
struct Point {
    x: u32,
    y: u32,
}

trait TMovable {
    fn move(self: @Point, dx: u32, dy: u32) -> Point;
}

#[allow(clone_on_copy)]
impl Movable of TMovable {
    fn move(self: @Point, dx: u32, dy: u32) -> Point {
        let new_point = self.clone();
        Point { x: new_point.x + dx, y: new_point.y + dy }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1.move(5, 5);
    println!("{}, {}", p1.x, p2.y);
}
"#;

const ALLOW_CLONE_ARRAY: &str = r#"
fn main() {
    let arr: [u32; 3] = [1, 2, 3];
    #[allow(clone_on_copy)]
    let arr_clone = arr.clone();
    println!("{:?}", arr_clone);
}
"#;

#[test]
fn clone_numeric_type_diagnostic() {
    test_lint_diagnostics!(CLONE_NUMERIC_TYPE, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:4:13
        let b = a.clone();
                ^^^^^^^^^
    ")
}

#[test]
fn clone_numeric_type_fixer() {
    test_lint_fixer!(CLONE_NUMERIC_TYPE, @r#"
    fn main() {
        let a: u32 = 42;
        let b = a;
        println!("{}", b);
    }
    "#)
}

#[test]
fn clone_felt252_diagnostic() {
    test_lint_diagnostics!(CLONE_FELT252, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:4:13
        let b = a.clone();
                ^^^^^^^^^
    ");
}

#[test]
fn clone_felt252_fixer() {
    test_lint_fixer!(CLONE_FELT252, @r#"
    fn main() {
        let a: felt252 = 'hello'
        let b = a;
        println!("{}", b);
    }
    "#);
}
#[test]
fn clone_struct_diagnostic() {
    test_lint_diagnostics!(CLONE_STRUCT, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:10:14
        let p2 = p1.clone();
                 ^^^^^^^^^^
    ");
}

#[test]
fn clone_struct_fixer() {
    test_lint_fixer!(CLONE_STRUCT, @r##"
    #[derive(Copy, Drop)]
    struct Point {
        x: u32,
        y: u32,
    }

    fn main() {
        let p1 = Point { x: 10, y: 20 };
        let p2 = p1;
        println!("{}, {}", p1.x, p2.y);
    }
    "##);
}

#[test]
fn clone_non_copy_struct_diagnostic() {
    test_lint_diagnostics!(CLONE_NON_COPY_STRUCT, @r"
    ");
}

#[test]
fn clone_non_copy_struct_fixer() {
    test_lint_fixer!(CLONE_NON_COPY_STRUCT, @r##"
    #[derive(Clone, Drop)]
    struct Point {
        x: u32,
        y: u32,
    }

    fn main() {
        let p1 = Point { x: 10, y: 20 };
        let p2 = p1.clone();
        println!("{}, {}", p1.x, p2.y);
    }
    "##);
}

#[test]
fn clone_tuple_diagnostic() {
    test_lint_diagnostics!(CLONE_TUPLE, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:4:19
        let t_clone = t.clone();
                      ^^^^^^^^^
    ");
}

#[test]
fn clone_tuple_fixer() {
    test_lint_fixer!(CLONE_TUPLE, @r#"
    fn main() {
        let t: (u32, felt252) = (42, 'hello');
        let t_clone = t;
        println!("{:?}", t_clone);
    }
    "#);
}
#[test]
fn clone_array_diagnostic() {
    test_lint_diagnostics!(CLONE_ARRAY, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:4:21
        let arr_clone = arr.clone();
                        ^^^^^^^^^^^
    ");
}

#[test]
fn clone_array_fixer() {
    test_lint_fixer!(CLONE_ARRAY, @r#"
    fn main() {
        let arr: [u32; 3] = [1, 2, 3];
        let arr_clone = arr;
        println!("{:?}", arr_clone);
    }
    "#);
}

#[test]
fn clone_in_impl_diagnostic() {
    test_lint_diagnostics!(CLONE_IN_IMPL, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:14:25
            let new_point = self.clone();
                            ^^^^^^^^^^^^
    ")
}

#[test]
fn clone_in_impl_fixer() {
    test_lint_fixer!(CLONE_IN_IMPL, @r##"
    #[derive(Copy, Drop)]
    struct Point {
        x: u32,
        y: u32,
    }

    trait TMovable {
        fn move(self: @Point, dx: u32, dy: u32) -> Point;
    }

    impl Movable of TMovable {
        fn move(self: @Point, dx: u32, dy: u32) -> Point {
            let new_point = *self;
            Point { x: new_point.x + dx, y: new_point.y + dy }
        }
    }

    fn main() {
        let p1 = Point { x: 10, y: 20 };
        let p2 = p1.move(5, 5);
        println!("{}, {}", p1.x, p2.y);
    }
    "##);
}

#[test]
fn clone_on_function_diagnostic() {
    test_lint_diagnostics!(CLONE_ON_FUNCTION, @r"
    Plugin diagnostic: using `clone` on type which implements Copy trait
     --> lib.cairo:7:13
        let b = some_function().clone();
                ^^^^^^^^^^^^^^^^^^^^^^^
    ");
}

#[test]
fn clone_on_function_fixer() {
    test_lint_fixer!(CLONE_ON_FUNCTION, @r#"
    fn some_function() -> u32 {
        42
    }

    fn main() {
        let b = some_function();
        println!("{}", b);
    }
    "#)
}
#[test]
fn allow_clone_in_impl_diagnostic() {
    test_lint_diagnostics!(ALLOW_CLONE_IN_IMPL, @r"")
}

#[test]
fn allow_clone_array_diagnostics() {
    test_lint_diagnostics!(ALLOW_CLONE_ARRAY, @r"")
}

#[test]
fn allow_clone_in_impl_fixer() {
    test_lint_diagnostics!(ALLOW_CLONE_IN_IMPL, @r"")
}

#[test]
fn clone_with_snapshot() {
    test_lint_fixer!(CLONE_WITH_SNAPSHOT, @r##"
    #[derive(Copy, Drop)]
    struct Point {
        x: u32,
        y: u32,
    }

    fn duplicate(point: @Point) -> Point {
        let cloned_point = *point;
        cloned_point
    }

    fn main() {
        let p1 = Point { x: 10, y: 20 };
        let p2 = duplicate(@p1);

        println!("Original: ({}, {}), Cloned: ({}, {})", p1.x, p1.y, p2.x, p2.y);
    }
    "##)
}
