# equality_match

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/single_match.rs#L86)

## What it does

Checks for matches that do something only in 1 arm and can be rewrote as an `if`

## Example

```cairo
match variable {
    Option::None => println!("None"),
    Option::Some => (),
};
```

Which can be probably rewritten as

```cairo
if variable.is_none() {
    println!("None");
}
```
