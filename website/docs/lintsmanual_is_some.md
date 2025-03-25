# manual_is_some

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_is.rs#L40)

## What it does

Checks for manual implementations of `is_some`.

## Example

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = match foo {
        Option::Some(_) => true,
        Option::None => false,
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Option<i32> = Option::None;
    let _foo = foo.is_some();
}
```
