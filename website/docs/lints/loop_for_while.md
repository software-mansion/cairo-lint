# loop_for_while

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/loops/loop_for_while.rs#L48)

## What it does

Checks for `loop` expressions that contain a conditional `if` statement with break inside that
can be simplified to a `while` loop.

## Example

```cairo
fn main() {
    let mut x: u16 = 0;
    loop {
        if x == 10 {
            break;
        }
        x += 1;
    }
}
```

Can be simplified to:

```cairo
fn main() {
    let mut x: u16 = 0;
    while x != 10 {
        x += 1;
    }
}
```
