# eq_comp_op

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/eq_op.rs#L72)

## What it does

Checks for comparison with identical operands.

## Example

```cairo
fn foo(a: u256) -> bool {
    a == a
}
```

Could be simplified by replacing the entire expression with true:

```cairo
fn foo(a: u256) -> bool {
    true
}
```