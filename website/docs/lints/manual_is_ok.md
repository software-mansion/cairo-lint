# manual_is_ok

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/manual/manual_is.rs#L136)

## What it does

Checks for manual implementations of `is_ok`.

## Example

```cairo
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(_) => true,
        Result::Err(_) => false
    };
}
```

Can be rewritten as:

```cairo
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = res_val.is_ok();
}
```
