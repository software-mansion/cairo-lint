use cairo_lang_defs::{
    ids::{FunctionWithBodyId, LanguageElementId, ModuleId, ModuleItemId},
    plugin::PluginDiagnostic,
};
use cairo_lang_diagnostics::Severity;
use cairo_lang_filesystem::ids::FileLongId;
use cairo_lang_lowering::{
    self as lowering, Lowered, LoweringStage, Statement, StatementCall, db::LoweringGroup,
};
use cairo_lang_parser::db::ParserGroup;
use cairo_lang_semantic::{
    self as semantic, FunctionId,
    corelib::{core_bool_enum, unit_ty},
    helper::ModuleHelper,
    items::{imp::ImplSemantic, trt::TraitSemantic},
    lsp_helpers::LspHelpers,
};
use cairo_lang_syntax::node::{
    SyntaxNode, Terminal, TypedSyntaxNode,
    ast::{ExprInlineMacro, ExprUnary, PathSegment},
};
use cairo_lang_utils::Intern;
use itertools::Itertools;
use salsa::Database;

use crate::{
    context::{CairoLintKind, Lint},
    queries::get_all_inline_macro_calls,
};

pub struct AssertOnConst;

/// ## What it does
///
/// Checks for assertions on boolean literals, constants and expressions
/// which are simplified to constants by the compiler.
///
/// ## Example
///
/// ```cairo
/// fn main() {
///     // Bool consts:
///     const C: bool = true;
///     assert!(C);  // Always passes
///
///     // Bool literals:
///     assert!(true);  // Always passes
///     assert!(false);  // Never passes
///
///     // Bool expressions:
///     assert!(true && false);  // Never passes
///     assert!((1 == 1) || (2 == 2));  // Always passes
/// }
/// ```
impl Lint for AssertOnConst {
    fn allowed_name(&self) -> &'static str {
        "assert_on_const"
    }

    fn diagnostic_message(&self) -> &'static str {
        "Unnecessary assert on a const value detected."
    }

    fn kind(&self) -> crate::context::CairoLintKind {
        CairoLintKind::RedundantOperation
    }
}

/// Checks for `assert!`s called on const boolean expressions.
///
/// This function implements an algorithm which allows us to determine whether an `assert!`
/// is called with an argument which can be simplified to a const expression by the compiler during const folding.
///
/// # Expansion of the `assert!` macro:
/// `assert!(condition, error_message)` generates the following code:
/// ```cairo
/// if !(condition) {
///     // panic with error_message
/// }
/// ```
///
/// This expansion has two important properties in two layers of abstraction:
/// 1. Syntax: AST of the expansion **always** contains a `ExprUnary` node as a top-level expression inside the `if`
/// 2. Semantic: This unary expression **always** translates to the call of `core::bool_not_impl` function on the condition.
///
/// # Algorithm:
/// For each `assert!`:
/// 1. Obtain its expansion and the top-level unary expression.
/// 2. Obtain the lowered representation of the function.
/// 3. Collect all boolean consts and lowering-level variables which have unconditional, constant values.
/// 4. Collect all calls to `core::bool_not_impl` on those consts and const variables. **Important note**: Lowering representation contains their location in the code.
/// 5. Check if among the collected calls to `core::bool_not_impl` **exactly one** has a span identical to the unary expression from the `assert!` expansion.
///
/// This way, we make absolutely sure that the expression we call `assert!` on can be const-folded.
#[tracing::instrument(skip_all, level = "trace")]
pub fn check_assert_on_const<'db>(
    db: &'db dyn Database,
    item: &ModuleItemId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let functions_with_body = match item {
        ModuleItemId::FreeFunction(free_function_id) => {
            vec![FunctionWithBodyId::Free(*free_function_id)]
        }
        ModuleItemId::Impl(impl_def_id) => {
            let Ok(impl_functions) = db.impl_functions(*impl_def_id) else {
                return;
            };

            impl_functions
                .values()
                .map(|impl_function_id| FunctionWithBodyId::Impl(*impl_function_id))
                .collect_vec()
        }
        ModuleItemId::Trait(trait_id) => {
            let Ok(trait_functions) = db.trait_functions(*trait_id) else {
                return;
            };

            trait_functions
                .values()
                .map(|trait_function_id| FunctionWithBodyId::Trait(*trait_function_id))
                .collect_vec()
        }
        _ => return,
    };

    for function_with_body_id in functions_with_body {
        check_assert_on_const_for_function_with_body(db, item, function_with_body_id, diagnostics);
    }
}

fn check_assert_on_const_for_function_with_body<'db>(
    db: &'db dyn Database,
    module_item_id: &ModuleItemId<'db>,
    function_with_body_id: FunctionWithBodyId<'db>,
    diagnostics: &mut Vec<PluginDiagnostic<'db>>,
) {
    let Some(function_body_lowering) = get_function_body_lowering(db, function_with_body_id) else {
        return;
    };

    let bool_not_impl_calls_on_const_exprs =
        find_bool_not_impl_calls_on_const_values(db, function_body_lowering);

    let module_id = module_item_id.parent_module(db);

    for assert_call in get_assert_macro_calls(db, module_item_id) {
        let Some(expansion_syntax) = get_inline_macro_expansion_syntax(db, &assert_call, module_id)
        else {
            continue;
        };

        let Some(unary_expression) = expansion_syntax
            .descendants(db)
            .find_map(|node| node.cast::<ExprUnary>(db))
        else {
            continue;
        };

        if is_unary_expr_a_bool_not_impl_call_on_const(
            db,
            unary_expression,
            &bool_not_impl_calls_on_const_exprs,
        ) {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: assert_call.as_syntax_node().stable_ptr(db),
                message: AssertOnConst.diagnostic_message().to_string(),
                severity: Severity::Warning,
                inner_span: None,
                error_code: None,
            })
        }
    }
}

/// Returns a lowered representation of the function after applying baseline optimizations,
/// including the const folding, which is essential for this lint.
fn get_function_body_lowering<'db>(
    db: &'db dyn Database,
    function_with_body_id: FunctionWithBodyId<'db>,
) -> Option<&'db Lowered<'db>> {
    let semantic_concrete_function_with_body_id =
        semantic::ConcreteFunctionWithBodyId::from_generic(db, function_with_body_id).ok()?;

    let lowering_concrete_function_with_body_id =
        lowering::ids::ConcreteFunctionWithBodyLongId::Semantic(
            semantic_concrete_function_with_body_id,
        )
        .intern(db);

    db.lowered_body(
        lowering_concrete_function_with_body_id,
        LoweringStage::PostBaseline,
    )
    .ok()
}

/// Finds all statements in the lowered representation which are calls to `core::bool_not_impl`.
/// Returns only those which have **constant arguments**.
fn find_bool_not_impl_calls_on_const_values<'db>(
    db: &'db dyn Database,
    function_body_lowering: &'db Lowered<'db>,
) -> Vec<&'db StatementCall<'db>> {
    // Const statements can be collected from all blocks.
    // They are rather unlikely to appear outside the block they are defined in though.
    let mut const_statements = vec![];

    // We collect all these calls from all the blocks because
    // we never know which of them is a part of the assert! macro.
    let mut bool_not_impl_calls_on_const_exprs: Vec<&'db StatementCall<'db>> = vec![];

    for (_, block) in function_body_lowering.blocks.iter() {
        // Unit structs and bool enums should be collected separately for each block.
        // If variables with those are used across two blocks,
        // it always means that their values are conditional.
        let mut unit_structs = vec![];
        let mut bool_enum_constructs = vec![];

        for statement in block.statements.iter() {
            match statement {
                // We obviously need to collect all bool constants.
                // Those are defined explicitly by the user and aren't results of the const folding.
                Statement::Const(const_statement) => {
                    const_statements.push(const_statement);
                }

                // Bool enums are always constructed from unit structs,
                // that's why we need to collect those.
                Statement::StructConstruct(struct_construct) => {
                    let output_variable = function_body_lowering
                        .variables
                        .get(struct_construct.output)
                        .expect("function body lowering should contain variable from it own block");

                    let output_type = output_variable.ty.long(db);

                    if output_type == unit_ty(db).long(db) {
                        unit_structs.push(struct_construct);
                    }
                }

                // Collect all bool enum instantiations.
                Statement::EnumConstruct(enum_construct) => {
                    let input_variable = enum_construct.input;

                    let concrete_enum_id = enum_construct.variant.concrete_enum_id;
                    let is_bool = concrete_enum_id == core_bool_enum(db);

                    // Bool enum can also be constructed from other boolean variable.
                    // That means it is not constant.
                    let is_constructed_from_unit_struct = unit_structs
                        .iter()
                        .any(|unit_struct| unit_struct.output == input_variable.var_id);

                    if is_constructed_from_unit_struct && is_bool {
                        bool_enum_constructs.push(enum_construct);
                    }
                }

                // Collect all calls to `core::bool_not_impl`.
                Statement::Call(call) => {
                    let Some(function_id) =
                        try_semantic_function_id_from_lowering(db, call.function)
                    else {
                        continue;
                    };

                    if !is_core_bool_not_impl(db, function_id) {
                        continue;
                    }

                    // This function is guaranteed to have exactly one argument.
                    let input = call
                        .inputs
                        .first()
                        .expect("bool_not_impl should have exactly one argument");

                    // Check if the function is called on a bool constant (defined as Statement::Const).
                    let is_input_const = const_statements
                        .iter()
                        .any(|const_statement| const_statement.output == input.var_id);

                    // Check if the function is called on a bool variable, created using `EnumConstruct`.
                    let is_input_bool_literal = bool_enum_constructs
                        .iter()
                        .any(|bool_enum| bool_enum.output == input.var_id);

                    // Check if the function is called on an output of other `bool_not_impl` captured before.
                    // This occurs when `assert!` contains a negated expression.
                    let is_input_other_bool_not_impl = bool_not_impl_calls_on_const_exprs
                        .iter()
                        .any(|call| call.outputs.contains(&input.var_id));

                    if !is_input_const && !is_input_bool_literal && !is_input_other_bool_not_impl {
                        continue;
                    }

                    bool_not_impl_calls_on_const_exprs.push(call);
                }

                _ => {}
            }
        }
    }

    bool_not_impl_calls_on_const_exprs
}

/// Transforms a function ID from the lowering representation to the corresponding semantic representation.
fn try_semantic_function_id_from_lowering<'db>(
    db: &'db dyn Database,
    function_id: lowering::ids::FunctionId<'db>,
) -> Option<semantic::FunctionId<'db>> {
    match function_id.long(db) {
        lowering::ids::FunctionLongId::Semantic(function_id) => Some(*function_id),
        _ => None,
    }
}

/// Checks if the given function is `core::bool_not_impl`.
fn is_core_bool_not_impl<'db>(db: &'db dyn Database, function_id: FunctionId<'db>) -> bool {
    let bool_not_impl = ModuleHelper::core(db).extern_function_id("bool_not_impl");
    function_id.try_get_extern_function_id(db) == Some(bool_not_impl)
}

/// Returns all `assert!` calls from the given module item.
fn get_assert_macro_calls<'db>(
    db: &'db dyn Database,
    module_item: &ModuleItemId<'db>,
) -> impl Iterator<Item = ExprInlineMacro<'db>> {
    get_all_inline_macro_calls(db, module_item)
        .into_iter()
        .filter(|call| {
            let path_elements = call.path(db).segments(db).elements(db).collect::<Vec<_>>();
            match &path_elements[..] {
                [PathSegment::Simple(path_segment)] => {
                    path_segment.ident(db).text(db).long(db) == "assert"
                }
                _ => false,
            }
        })
}

/// Returns a syntax node which is a root of the syntax tree
/// constructed during expansion of the given inline macro.
fn get_inline_macro_expansion_syntax<'db>(
    db: &'db dyn Database,
    inline_macro: &ExprInlineMacro<'db>,
    module: ModuleId<'db>,
) -> Option<SyntaxNode<'db>> {
    let expansion_virtual_file =
        db.inline_macro_expansion_files(module)
            .iter()
            .find_map(|&file_id| {
                let FileLongId::Virtual(virtual_file) = file_id.long(db) else {
                    return None;
                };

                let span = virtual_file.parent?;

                (inline_macro.as_syntax_node().span(db).contains(span.span))
                    .then_some(file_id)
                    .or(None)
            })?;

    db.file_syntax(expansion_virtual_file).ok()
}

/// Checks if the given list of calls to `core::bool_not_impl` function
/// contains exactly one call which was generated by the `assert!` macro.
fn is_unary_expr_a_bool_not_impl_call_on_const<'db>(
    db: &'db dyn Database,
    unary_expr: ExprUnary<'db>,
    bool_not_impl_calls: &[&'db StatementCall<'db>],
) -> bool {
    let bool_not_calls_inside_assert = bool_not_impl_calls
        .iter()
        .filter(|call| {
            call.inputs
                .first()
                .expect("bool_not_impl should have exactly one argument")
                .location
                .all_locations(db)
                .iter()
                .any(|location| {
                    location.span_in_file(db)
                        == unary_expr.as_syntax_node().stable_ptr(db).span_in_file(db)
                })
        })
        .collect::<Vec<_>>();

    bool_not_calls_inside_assert.len() == 1
}
