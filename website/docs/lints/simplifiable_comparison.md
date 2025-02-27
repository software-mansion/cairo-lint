# simplifiable_comparison

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/double_comparison.rs#L85)

## What it does

Checks for double comparisons that can be simplified.
Those are comparisons that can be simplified to a single comparison.

## Example

```cairo
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x == y || x > y {
        true
    } else {
        false
    }
}
```

The above code can be simplified to:

```cairo
fn main() -> bool {
    let x = 5_u32;
    let y = 10_u32;
    if x >= y {
        true
    } else {
        false
    }
}
```