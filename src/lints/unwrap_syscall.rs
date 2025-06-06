use crate::{
    context::{CairoLintKind, Lint},
    fixes::InternalFix,
    helper::find_module_file_containing_node,
    queries::{get_all_function_bodies, get_all_function_calls},
    types::format_type,
};
use cairo_lang_defs::{ids::ModuleItemId, plugin::PluginDiagnostic};
use cairo_lang_diagnostics::Severity;
use cairo_lang_semantic::{db::SemanticGroup, Arenas, ExprFunctionCall, ExprFunctionCallArg};
use cairo_lang_syntax::node::{ast, SyntaxNode, TypedStablePtr, TypedSyntaxNode};
use itertools::Itertools;

pub struct UnwrapSyscall;

const SYSCALL_RESULT_TYPE: &str = "Result<felt252, Array<felt252>>";
const RESULT_CORE_PATH: &str = "core::result::Result";
const UNWRAP_PATH_BEGINNING: &str = "core::result::ResultTraitImpl::<";
const UNWRAP_PATH_END: &str = ">::unwrap";
const UNWRAP_SYSCALL_TRAIT_PATH: &str = "starknet::SyscallResultTrait";

/// ## What it does
///
/// Detects if the function uses `unwrap` on a `SyscallResult` object.
///
/// ## Example
///
/// ```cairo
/// use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
/// use starknet::syscalls::storage_read_syscall;
///
/// fn main() {
///     let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
///     let result = storage_read_syscall(0, storage_address_from_base(storage_address));
///     result.unwrap();
/// }
/// ```
///
/// Can be changed to:
///
/// ```cairo
/// use starknet::SyscallResultTrait;
/// use starknet::storage_access::{storage_address_from_base, storage_base_address_from_felt252};
/// use starknet::syscalls::storage_read_syscall;
///
/// fn main() {
///     let storage_address = storage_base_address_from_felt252(3534535754756246375475423547453);
///     let result = storage_read_syscall(0, storage_address_from_base(storage_address));
///     result.unwrap_syscall();
/// }
/// ```
impl Lint for UnwrapSyscall {
    fn allowed_name(&self) -> &'static str {
        "unwrap_syscall"
    }

    fn diagnostic_message(&self) -> &'static str {
        "consider using `unwrap_syscall` instead of `unwrap`"
    }

    fn kind(&self) -> CairoLintKind {
        CairoLintKind::UnwrapSyscall
    }

    fn has_fixer(&self) -> bool {
        true
    }

    fn fix(&self, db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
        fix_unwrap_syscall(db, node)
    }

    fn fix_message(&self) -> Option<&'static str> {
        Some("Replace with `unwrap_syscall()` for syscall results")
    }
}

pub fn check_unwrap_syscall(
    db: &dyn SemanticGroup,
    item: &ModuleItemId,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_bodies = get_all_function_bodies(db, item);
    for function_body in function_bodies.iter() {
        let function_call_exprs = get_all_function_calls(function_body);
        let arenas = &function_body.arenas;
        for function_call_expr in function_call_exprs {
            check_single_unwrap_syscall(db, &function_call_expr, arenas, diagnostics);
        }
    }
}

fn check_single_unwrap_syscall(
    db: &dyn SemanticGroup,
    expr: &ExprFunctionCall,
    arenas: &Arenas,
    diagnostics: &mut Vec<PluginDiagnostic>,
) {
    let function_name = expr.function.get_concrete(db).generic_function.format(db);

    if !function_name.starts_with(UNWRAP_PATH_BEGINNING)
        || !function_name.ends_with(UNWRAP_PATH_END)
    {
        return;
    }

    if expr.args.is_empty() {
        return;
    }

    match expr.args.first().unwrap() {
        ExprFunctionCallArg::Reference(_) => (),
        ExprFunctionCallArg::Value(id) => {
            let expr = &arenas.exprs[*id];
            let type_name = expr.ty().short_name(db).split("::").take(3).join("::");
            let node = expr.stable_ptr().lookup(db).as_syntax_node();
            let module_file_id = match find_module_file_containing_node(db, &node) {
                Some(id) => id,
                None => return,
            };
            let importables = db
                .visible_importables_from_module(module_file_id)
                .unwrap_or_else(|| panic!("Couldn't find importables for {:?}", node));

            let formatted_type = format_type(db, expr.ty(), &importables);
            if formatted_type == SYSCALL_RESULT_TYPE && type_name == RESULT_CORE_PATH {
                diagnostics.push(PluginDiagnostic {
                    stable_ptr: expr
                        .stable_ptr()
                        .lookup(db)
                        .as_syntax_node()
                        .parent(db)
                        .unwrap()
                        .stable_ptr(db),
                    message: UnwrapSyscall.diagnostic_message().to_string(),
                    severity: Severity::Warning,
                    relative_span: None,
                    inner_span: None,
                })
            }
        }
    }
}

fn fix_unwrap_syscall(db: &dyn SemanticGroup, node: SyntaxNode) -> Option<InternalFix> {
    let ast_expr_binary = ast::ExprBinary::cast(db, node).unwrap_or_else(|| {
        panic!(
          "Expected a binary expression for unwrap called on SyscallResult. Actual node text: {:?}",
          node.get_text(db)
        )
    });

    let fixed = format!(
        "{}{}unwrap_syscall()",
        ast_expr_binary.lhs(db).as_syntax_node().get_text(db),
        ast_expr_binary.op(db).as_syntax_node().get_text(db)
    );
    Some(InternalFix {
        node,
        suggestion: fixed,
        description: UnwrapSyscall.fix_message().unwrap().to_string(),
        import_addition_paths: Some(vec![UNWRAP_SYSCALL_TRAIT_PATH.to_string()]),
    })
}
