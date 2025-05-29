# collapsible_if_else

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/ifs/collapsible_if_else.rs#L54)

## What it does

Checks for nested `if` statements inside the `else` statement
that can be collapsed into a single `if-else` statement.

## Example

```cairo
fn main() {
    let x = true;
    if x {
        println!("x is true");
    } else {
        if !x {
            println!("x is false");
        }
    }
}
```

Can be refactored to:

```cairo
fn main() {
    let x = true;
    if x {
        println!("x is true");
    } else if !x {
        println!("x is false");
    }
}
```
