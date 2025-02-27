# equatable_if_let

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/ifs/equatable_if_let.rs#L36)

## What it does

Checks for `if let` pattern matching that can be replaced by a simple comparison.

## Example

```cairo
if let Some(2) = a {
    // Code
}
```

Could be replaced by

```cairo
if a == Some(2) {
    // Code
}
````