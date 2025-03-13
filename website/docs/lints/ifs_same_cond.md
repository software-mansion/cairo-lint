# ifs_same_cond

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/ifs/ifs_same_cond.rs#L43)

## What it does

Checks for consecutive `if` expressions with the same condition.

## Example

```cairo
fn main() {
    let a = 1;
    let b = 1;
    if a == b {
        println!("a is equal to b");
    } else if a == b {
        println!("a is equal to b");
    }
}
```

Could be rewritten as just:

```cairo
fn main() {
    let a = 1;
    let b = 1;
    if a == b {
        println!("a is equal to b");
    }
}
```
