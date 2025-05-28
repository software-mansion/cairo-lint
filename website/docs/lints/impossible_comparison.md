# impossible_comparison

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/double_comparison.rs#L38)

## What it does

Checks for impossible comparisons. Those ones always return false.

## Example

Here is an example of impossible comparison:

```cairo
fn main() {
    let x: u32 = 1;
    if x > 200 && x < 100 {
        //impossible to reach
    }
}
```
