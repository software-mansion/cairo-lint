# break_unit

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/breaks.rs#L39)

## What it does

Checks for `break ();` statements and suggests removing the parentheses.

## Example

```cairo
fn main() {
    loop {
        break ();
    }
}
```

Can be fixed by removing the parentheses:

```cairo
fn main() {
    loop {
        break;
    }
}
```
