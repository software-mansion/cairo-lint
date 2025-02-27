# bitwise_for_parity_check

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/bitwise_for_parity_check.rs#L28)

## What it does

Checks for `x & 1` which is unoptimized in cairo and could be replaced by `x % 1`.

## Example

```cairo
fn main() {
    let _a = 200_u32 & 1;
}
```