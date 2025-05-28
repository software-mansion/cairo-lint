use crate::{test_lint_diagnostics, test_lint_fixer};

const BASIC_SYSCALL: &str = r#"
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap();
}
"#;

const BASIC_SYSCALL_ASSIGN: &str = r#"
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    let _a = result.unwrap();
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

const ALREADY_IMPORTED_SYSCALL_RESULT_TRAIT: &str = r#"
use starknet::SyscallResultTrait; // Already imported
use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
use starknet::syscalls::storage_read_syscall;

fn main() {
    let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
    let result = storage_read_syscall(0, storage_address_from_base(storage_address));
    result.unwrap();
}
"#;

#[test]
fn test_basic_syscall_fixer() {
    test_lint_fixer!(BASIC_SYSCALL, @r"
    use starknet::SyscallResultTrait;
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        result.unwrap_syscall();
    }
    ");
}

#[test]
fn test_basic_syscall_assign_diagnostics() {
    test_lint_diagnostics!(BASIC_SYSCALL_ASSIGN, @r"
    Plugin diagnostic: consider using `unwrap_syscall` instead of `unwrap`
     --> lib.cairo:8:14
        let _a = result.unwrap();
                 ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn test_basic_syscall_assign_fixer() {
    test_lint_fixer!(BASIC_SYSCALL_ASSIGN, @r"
    use starknet::SyscallResultTrait;
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        let _a = result.unwrap_syscall();
    }
    ");
}

#[test]
fn test_basic_syscall_allowed_diagnostics() {
    test_lint_diagnostics!(BASIC_SYSCALL_ALLOWED, @"");
}

#[test]
fn test_basic_syscall_allowed_fixer() {
    test_lint_fixer!(BASIC_SYSCALL_ALLOWED, @r"
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        #[allow(unwrap_syscall)]
        result.unwrap();
    }
    ");
}

#[test]
fn test_correct_syscall_unwrap_diagnostics() {
    test_lint_diagnostics!(CORRECT_SYSCALL_UNWRAP, @"");
}

#[test]
fn test_correct_syscall_unwrap_fixer() {
    test_lint_fixer!(CORRECT_SYSCALL_UNWRAP, @r"
    use starknet::SyscallResultTrait;
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        result.unwrap_syscall();
    }
    ");
}

#[test]
fn already_imported_syscall_result_trait_diagnostics() {
    test_lint_diagnostics!(ALREADY_IMPORTED_SYSCALL_RESULT_TRAIT, @r"
    Unused import: `test::SyscallResultTrait`
     --> lib.cairo:2:15
    use starknet::SyscallResultTrait; // Already imported
                  ^^^^^^^^^^^^^^^^^^
    Plugin diagnostic: consider using `unwrap_syscall` instead of `unwrap`
     --> lib.cairo:9:5
        result.unwrap();
        ^^^^^^^^^^^^^^^
    ");
}

#[test]
fn already_imported_syscall_result_trait_fixer() {
    test_lint_fixer!(ALREADY_IMPORTED_SYSCALL_RESULT_TRAIT, @r"
    use starknet::SyscallResultTrait; // Already imported
    use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
    use starknet::syscalls::storage_read_syscall;

    fn main() {
        let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
        let result = storage_read_syscall(0, storage_address_from_base(storage_address));
        result.unwrap_syscall();
    }
    ");
}
