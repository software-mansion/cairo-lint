# enum_variant_names

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/enum_variant_names.rs#L36)

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
