# div_eq_op

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/eq_op.rs#L36)

## What it does

Checks for division with identical operands.

## Example

```cairo
fn foo(a: u256) -> u256 {
    a / a
}
```

Could be simplified by replacing the entire expression with 1:

```cairo
fn foo(a: u256) -> u256 {
    1
}
```
