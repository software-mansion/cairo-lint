# double_parens

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/double_parens.rs#L34)

## What it does

Checks for unnecessary double parentheses in expressions.

## Example

```cairo
fn main() -> u32 {
    ((0))
}
```

Can be simplified to:

```cairo
fn main() -> u32 {
    0
}
```
