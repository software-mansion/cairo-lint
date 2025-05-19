# unit_return_type

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/unit_return_type.rs#L32)

## What it does

Detects if the function has a unit return type, which is not needed to be specified.

## Example

```cairo
fn foo() -> () {
    println!("Hello, world!");
}
```

Can be simplified to just:

```cairo
fn foo() {
    println!("Hello, world!");
}
```
