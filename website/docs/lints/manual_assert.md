# manual_assert

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_assert.rs#L49)

## What it does

Checks for manual implementations of `assert` macro in `if` expressions.

## Example

```cairo
fn main() {
    let a = 5;
    if a == 5 {
        panic!("a shouldn't be equal to 5");
    }
}
```

Can be rewritten as:

```cairo
fn main() {
    let a = 5;
    assert!(a != 5, "a shouldn't be equal to 5");
}
```
