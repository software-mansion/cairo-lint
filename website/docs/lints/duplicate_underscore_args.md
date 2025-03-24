# duplicate_underscore_args

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/duplicate_underscore_args.rs#L23)

## What it does

Checks for functions that have the same argument name but prefix with `_`.

## Example

This code will raise a warning because it can be difficult to differentiate between `test` and `_test`.

```cairo
fn foo(test: u32, _test: u32) {}
```
