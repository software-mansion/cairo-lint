# contradictory_comparison

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/double_comparison.rs#L188)

## What it does

Checks for double comparisons that are contradictory. Those are comparisons that are always false.

## Example

```cairo
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x < y && x > y {
        true
    } else {
        false
    }
}
```

Could be simplified to just:

```cairo
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    false
}
```
