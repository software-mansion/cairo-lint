# manual_ok

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/manual/manual_ok.rs#L40)

## What it does

Checks for manual implementation of `ok` method in match and if expressions.

## Example

```cairo
fn main() {
    let res_val: Result<i32> = Result::Err('err');
    let _a = match res_val {
        Result::Ok(x) => Option::Some(x),
        Result::Err(_) => Option::None,
    };
}
```

Can be replaced with:

```cairo
    let res_val: Result<i32> = Result::Err('err');
    let _a = res_val.ok();
}
```