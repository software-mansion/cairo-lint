# manual_ok_or

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_ok_or.rs#L51)

## What it does

Checks for manual implementations of ok_or.

## Example

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = match foo {
        Option::Some(v) => Result::Ok(v),
        Option::None => Result::Err('this is an err'),
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = foo.ok_or('this is an err');
}
```
