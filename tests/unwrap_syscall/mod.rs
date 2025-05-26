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

const BASIC_SYSCALL_ALLOWED: &str = r#"
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    #[allow(unwrap_syscall)]
    result.unwrap();
}
"#;

const CORRECT_SYSCALL_UNWRAP: &str = r#"
use starknet::SyscallResultTrait;
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap_syscall();
}
"#;

const CORRECT_SYSCALL_REF_UNWRAP: &str = r#"
use starknet::SyscallResultTrait;
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap_syscall();
}
"#;

#[test]
fn test_basic_syscall_diagnostics() {
    test_lint_diagnostics!(BASIC_SYSCALL, @r"
    Plugin diagnostic: consider using `unwrap_syscall` instead of `unwrap`
     --> lib.cairo:8:5
        result.unwrap();
        ^^^^^^
    ");
}

#[test]
fn test_basic_syscall_allowed_diagnostics() {
    test_lint_diagnostics!(BASIC_SYSCALL_ALLOWED, @"");
}

#[test]
fn test_correct_syscall_unwrap_diagnostics() {
    test_lint_diagnostics!(CORRECT_SYSCALL_UNWRAP, @"");
}

#[test]
fn test_correct_syscall_ref_unwrap_diagnostics() {
    test_lint_diagnostics!(CORRECT_SYSCALL_REF_UNWRAP, @"");
}
