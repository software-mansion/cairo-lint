# redundant_op

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/redundant_op.rs#L36)

## What it does

Checks for redundant arithmetic operations like `x + 0`, `x - 0`, `x * 1`, `x / 1`

## Example

```cairo
fn main() {
    let x = 42;
    let _y = x * 1;
}
```

Can be simplified to

```cairo
fn main() {
    let x = 42;
    let _y = x;
}
```
