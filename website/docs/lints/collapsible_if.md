# collapsible_if

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/ifs/collapsible_if.rs#L51)

## What it does

Checks for nested `if` statements that can be collapsed into a single `if` statement.

## Example

```cairo
fn main() {
    let x = true;
    let y = true;
    let z = false;

    if x || z {
        if y && z {
            println!("Hello");
        }
    }
}
```

Can be collapsed to

```cairo
fn main() {
    let x = true;
    let y = true;
    let z = false;
    if (x || z) && (y && z) {
        println!("Hello");
    }
}
```
