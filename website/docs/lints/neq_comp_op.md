# neq_comp_op

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/eq_op.rs#L111)

## What it does

Checks for arithmetical comparison with identical operands.

## Example

```cairo
fn foo(a: u256) -> bool {
    let _z = a != a;
    let _y = a > a;
    a < a
}
```

Could be simplified by replacing the entire expression with false:

```cairo
fn foo(a: u256) -> bool {
    let _z = false;
    let _y = false;
    false
}
```
