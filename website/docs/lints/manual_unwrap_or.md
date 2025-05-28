# manual_unwrap_or

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_unwrap_or.rs#L39)

## What it does

Finds patterns that reimplement `Option::unwrap_or` or `Result::unwrap_or`.

## Example

```cairo
let foo: Option<i32> = None;
match foo {
    Some(v) => v,
    None => 1,
};
```

Can be simplified to:

```cairo
let foo: Option<i32> = None;
foo.unwrap_or(1);
```
