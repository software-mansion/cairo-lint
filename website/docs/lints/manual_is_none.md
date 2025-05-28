# manual_is_none

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_is.rs#L89)

## What it does

Checks for manual implementations of `is_none`.

## Example

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = match foo {
        Option::Some(_) => false,
        Option::None => true,
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = foo.is_none();
}
```
