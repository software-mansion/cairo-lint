# eq_diff_op

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/eq_op.rs#L146)

## What it does

Checks for subtraction with identical operands.

## Example

```cairo
fn foo(a: u256) -> u256 {
    a - a
}
```

Could be simplified by replacing the entire expression with zero:

```cairo
fn foo(a: u256) -> u256 {
    0
}
```
