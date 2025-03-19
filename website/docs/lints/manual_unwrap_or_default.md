# manual_unwrap_or_default

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_unwrap_or_default.rs#L47)

## What it does

Checks for manual unwrapping of an Option or Result.

## Example

```cairo
fn main() {
    let x: Option<u128> = Option::Some(1038);
    if let Option::Some(v) = x {
        v
    } else {
        0
    };
}
```

Can be simplified to:

```cairo
fn main() {
    let x: Option<u128> = Option::Some(1038);
    x.unwrap_or_default();
}
```
