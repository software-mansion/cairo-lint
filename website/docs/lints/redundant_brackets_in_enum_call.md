# redundant_brackets_in_enum_call

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/redundant_brackets_in_enum_call.rs#L42)

## What it does

Detects calls to enum variant constructors with redundant parentheses

## Example

```cairo
enum MyEnum {
    Data: u8,
    Empty,
}

fn main() {
    let a = MyEnum::Empty(()); // redundant parentheses
}
```

Can be simplified to:

```cairo
enum MyEnum {
    Data: u8,
    Empty,
}

fn main() {
    let a = MyEnum::Empty;
}
```
