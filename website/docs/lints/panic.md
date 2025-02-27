# panic

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/panic.rs#L28)

## What it does

Checks for panic usages.

## Example
```cairo
fn main() {
    panic!("panic");
}
```