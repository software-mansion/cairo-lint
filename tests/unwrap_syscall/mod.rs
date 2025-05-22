use crate::test_lint_diagnostics;

const BASIC_SYSCALL: &str = r#"
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap();
}
"#;

const SYSCALL_ON_REF: &str = r#"
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap();
}
"#;

#[test]
fn test_basic_syscall_diagnostics() {
    test_lint_diagnostics!(BASIC_SYSCALL, @r"
    Unwrap syscall. Consider using `unwrap_syscall` instead of `unwrap`.
     --> lib.cairo:5:5
         result.unwrap();
         ^^^^^^
    ");
}

#[test]
fn test_syscall_on_ref_diagnostics() {
    test_lint_diagnostics!(SYSCALL_ON_REF, @r"
    Unwrap syscall. Consider using `unwrap_syscall` instead of `unwrap`.
     --> lib.cairo:5:5
         result.unwrap();
         ^^^^^^
    ");
}
