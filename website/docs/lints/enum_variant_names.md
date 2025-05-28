# enum_variant_names

Default: **Disabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/enum_variant_names.rs#L37)

## What it does

Detects enumeration variants that are prefixed or suffixed by the same characters.

## Example

```cairo
enum Cake {
    BlackForestCake,
    HummingbirdCake,
    BattenbergCake,
}
```

Can be simplified to:

```cairo
enum Cake {
    BlackForest,
    Hummingbird,
    Battenberg,
}
```
