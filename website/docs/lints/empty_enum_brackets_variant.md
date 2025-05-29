# empty_enum_brackets_variant

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/empty_enum_brackets_variant.rs#L38)

## What it does

Finds enum variants that are declared with empty brackets.

## Example

```cairo
 enum MyEnum {
    Data: u8,
    Empty: ()       // redundant parentheses
 }
```

Can be simplified to:

```cairo
 enum MyEnum {
    Data(u8),
    Empty,
 }
```
