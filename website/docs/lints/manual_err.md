# manual_err

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_err.rs#L41)

## What it does

Checks for manual implementations of `err` in match and if expressions.

## Example

```cairo
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let _foo = match foo {
        Result::Ok(_) => Option::None,
        Result::Err(x) => Option::Some(x),
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Result<i32> = Result::Err('err');
    let _foo = foo.err();
}
```
