# erasing_op

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/erasing_op.rs#L41)

## What it does

Checks for operations that result in the value being erased (e.g., multiplication by 0 or 0 being divided by anything).

## Example

```cairo
fn main() {
    let x = 1;
    let _y = 0 * x;
    let _z = 0 / x;
    let _c = x & 0;
}
```

Could be simplified by replacing the entire expression with 0:

```cairo
fn main() {
    let x = 1;
    let _y = 0;
    let _z = 0;
    let _c = 0;
}
```