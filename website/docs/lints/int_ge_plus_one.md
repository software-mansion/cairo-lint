# int_ge_plus_one

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/int_op_one.rs#L40)

## What it does

Check for unnecessary add operation in integer >= comparison.

## Example

```cairo
fn main() {
    let x: u32 = 1;
    let y: u32 = 1;
    if x >= y + 1 {}
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
