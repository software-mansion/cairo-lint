# unwrap_syscall

Default: **Enabled**

[Source Code](https://github.com/software-mansion/cairo-lint/tree/main/src/lints/unwrap_syscall.rs#L50)

## What it does

Detects if the function uses `unwrap` on a `SyscallResult` object.

## Example

```cairo
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap();
}
```

Can be changed to:

```cairo
use starknet::SyscallResultTrait;
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap_syscall();
}
```
