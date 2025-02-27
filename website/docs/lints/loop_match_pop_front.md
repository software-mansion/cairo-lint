# loop_match_pop_front

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/crates/cairo-lint-core/src/lints/loops/loop_match_pop_front.rs#L52)

## What it does

Checks for loops that are used to iterate over a span using `pop_front`.

## Example

```cairo
let a: Span<u32> = array![1, 2, 3].span();
loop {
    match a.pop_front() {
        Option::Some(val) => {do_smth(val); },
        Option::None => { break; }
    }
}
```

Which can be rewritten as

```cairo
let a: Span<u32> = array![1, 2, 3].span();
for val in a {
    do_smth(val);
}
```