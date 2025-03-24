# manual_is_err

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_is.rs#L184)

## What it does

Checks for manual implementations of `is_err`.

## Example

```cairo
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(_) => false,
        Result::Err(_) => true
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = res_val.is_err();
}
```
