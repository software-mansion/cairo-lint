# clone_on_copy

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/clone_on_copy.rs#L33)

## What it does

Checks for usage of `.clone()` on a `Copy` type.

## Example

```cairo
    let a: felt252 = 'Hello';
    let b = a.clone()
```
