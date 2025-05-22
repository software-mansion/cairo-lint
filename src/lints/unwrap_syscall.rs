use std::any::Any;

use crate::{
    context::{CairoLintKind, Lint},
    helper::find_module_file_containing_node,
    queries::{get_all_function_bodies, get_all_function_calls},
    types::format_type,
};
use cairo_lang_defs::{
    ids::{ModuleItemId, TopLevelLanguageElementId},
    plugin::PluginDiagnostic,
};
use cairo_lang_semantic::{
    db::SemanticGroup, expr::inference::InferenceId, lsp_helpers::TypeFilter, resolve::Resolver, Arenas, ExprFunctionCall, ExprFunctionCallArg
};
use cairo_lang_syntax::node::{TypedStablePtr, TypedSyntaxNode};
use cairo_lang_utils::LookupIntern;

pub struct UnwrapSyscall;

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
    let function_name = expr.function.full_path(db);
    println!("Function name: {}", function_name);
    println!("Function args: {:?}", expr.args);

    if expr.args.len() == 0 {
        return;
    }

    match expr.args.get(0).unwrap() {
        ExprFunctionCallArg::Reference(_) => return,
        ExprFunctionCallArg::Value(id) => {
            let expr = &arenas.exprs[*id];
            let ty = expr.ty();
            let type_module_id = expr.ty().lookup_intern(db).module_id(db).unwrap();
            // let module_file_id = find_module_file_containing_node(db, &node)
            //     .expect(format!("Couldn't find module file for {:?}", node).as_str());
            let crate_id = type_module_id.owning_crate(db);
            let type_filter = match ty.head(db) {
                Some(head) => TypeFilter::TypeHead(head),
                None => TypeFilter::NoFilter,
            };
            let resolver = match lookup_item_id.and_then(|item| item.resolver_data(db).ok()) {
                Some(item) => Resolver::with_data(
                    db,
                    item.clone_with_inference_id(db, InferenceId::NoContext),
                ),
                None => Resolver::new(db, module_file_id, InferenceId::NoContext),
            };

            let methods = db.methods_in_crate(crate_id, type_filter.clone());
            println!("crate id: {:?}", crate_id);
            println!("crate modules: {:?}", db.crate_modules(crate_id));

            println!("Methods: {:?}", methods);
            for trait_function in methods.iter().copied() {
                println!("zydzi: {}", trait_function.trait_id(db).full_path(db));
            }
            // let importables = db
            //     .visible_importables_from_module(module_file_id)
            //     .expect(format!("Couldn't find importables for {:?}", node).as_str());

            // println!("Importables: {:?}", importables);
            // let _type = format_type(db, expr.ty(), &importables);
            // println!("Type: {:?}", expr.ty().lookup_intern(db));
            // println!("formated type: {:?}", _type);
        }
    }
    // let function = expr.function(db);
    // let function_name = function.name(db).to_string();
    // if function_name == "unwrap" {
    //     let stable_ptr = expr.stable_ptr().untyped();
    //     diagnostics.push(PluginDiagnostic::error(
    //         stable_ptr,
    //         "consider using `unwrap_syscall` instead of `unwrap`".to_string(),
    //     ));
    // }
}
