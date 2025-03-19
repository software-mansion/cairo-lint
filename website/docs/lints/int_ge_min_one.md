# int_ge_min_one

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/int_op_one.rs#L86)

## What it does

Check for unnecessary sub operation in integer >= comparison.

## Example

```cairo
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x - 1 >= y {}
}
```

Can be simplified to:

```cairo
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x > y {}
}
```
