# bool_comparison

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/bool_comparison.rs#L42)

## What it does

Checks for direct variable with boolean literal like `a == true` or `a == false`.

## Example

```cairo
fn main() {
    let x = true;
    if x == true {
        println!("x is true");
    }
}
```

Can be rewritten as:

```cairo
fn main() {
   let x = true;
   if x {
       println!("x is true");
   }
}
```
