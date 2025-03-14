# manual_expect_err

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_expect_err.rs#L53)

## What it does

Checks for manual implementation of `expect_err` method in match and if expressions.

## Example

```cairo
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let err = 'this is an err';
    let _foo = match foo {
        Result::Ok(_) => core::panic_with_felt252(err),
        Result::Err(x) => x,
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let err = 'this is an err';
    let _foo = foo.expect_err(err);
}
```
