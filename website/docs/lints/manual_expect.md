# manual_expect

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_expect.rs#L46)

## What it does

Checks for manual implementations of `expect`.

## Example

```cairo
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = match foo {
        Option::Some(x) => x,
        Option::None => core::panic_with_felt252('err'),
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let foo: Option::<i32> = Option::None;
    let _foo = foo.expect('err');
}
```
