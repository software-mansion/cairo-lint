# destruct_match

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/single_match.rs#L41)

## What it does

Checks for matches that do something only in 1 arm and can be rewrote as an `if let`

## Example

```cairo
let var = Option::Some(1_u32);
match var {
    Option::Some(val) => do_smth(val),
    _ => (),
}
```

Which can be rewritten as

```cairo
if let Option::Some(val) = var {
    do_smth(val),
}
```
