# inefficient_while_comp

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/performance.rs#L37)

## What it does

Checks if the while loop exit condition is using [`<`, `<=`, `>=`, `>`] operators.

## Example

```cairo
fn main() {
    let mut a = 1_u32;
    while a <= 10 {
        a += 1;
    }
}
```

Can be optimized to:

```cairo
fn main() {
    let mut a = 1_u32;
    while a != 10 {
        a += 1;
    }
}
```
