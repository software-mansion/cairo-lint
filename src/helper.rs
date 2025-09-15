//! # Helper Functions for Cairo Lint
//!
//! This module provides utility functions to assist in generating fixes for `if-else` conditions
//! within loops, inverting logical conditions, and processing code blocks.
//!
//! The main tasks of this module include:
//!
//! 1. Processing block and `else` clause content, including nested `if-else` constructs.
//! 2. Inverting logical conditions to their opposite for loop and condition rewriting.
//! 3. Skipping `break` statements when processing blocks to correctly transform loops.
//!
//! These helper functions can be reused in various parts of the Cairo Lint codebase to maintain
//! consistency and modularity when working with blocks and conditions.
use crate::LinterGroup;
use cairo_lang_defs::ids::{
    FileIndex, ImplItemId, LookupItemId, ModuleFileId, ModuleId, ModuleItemId, TraitItemId,
};
use cairo_lang_diagnostics::DiagnosticsBuilder;
use cairo_lang_filesystem::ids::{FileKind, FileLongId, VirtualFile};
use cairo_lang_formatter::{FormatterConfig, get_formatted_file};
use cairo_lang_parser::parser::Parser;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::imp::ImplSemantic;
use cairo_lang_semantic::items::module::ModuleSemantic;
use cairo_lang_semantic::items::trt::TraitSemantic;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCallArg, ExprId};
use cairo_lang_syntax::node::ast::{self, BlockOrIf, ElseClause, ExprBlock, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::Intern;
use if_chain::if_chain;
use num_bigint::BigInt;

pub const PANIC_PATH: &str = "core::panics::panic";
pub const PANIC_WITH_BYTE_ARRAY_PATH: &str = "core::panics::panic_with_byte_array";
pub const ASSERT_FORMATTER_NAME: &str = "__formatter_for_assert_macro__";
pub const ASSERT_PATH: &str = "core::fmt::Formatter";

/// Processes a block of code, formatting its content and ignoring any break statements.
///
/// # Arguments
///
/// * `db` - The syntax group which provides access to the syntax tree.
/// * `block` - The expression block (ExprBlock) to be processed.
/// * `indent` - A string representing the indentation to be applied to the block's content.
///
/// # Returns
///
/// A string representing the formatted content of the block.
///
/// # Example
///
/// Input: A block of code with the following statements:
/// ```cairo
/// let x = 5;
/// break;
/// let y = 10;
/// ```
/// Output: The formatted block without the `break` statement:
/// ```cairo
/// let x = 5;
/// let y = 10;
/// ```
///
/// This function skips the `break` statement and preserves the remaining statements in the block.
pub fn remove_break_from_block<'db>(
    db: &'db dyn SyntaxGroup,
    block: ExprBlock<'db>,
    indent: &str,
) -> String {
    let mut block_body = String::new();
    for statement in block.statements(db).elements(db) {
        if !matches!(statement, Statement::Break(_)) {
            let statement_code = statement.as_syntax_node().get_text(db);
            statement_code.trim().split("\n").for_each(|line| {
                block_body.push_str(&format!("{}    {}\n", indent, line.trim()));
            });
        }
    }
    block_body
}

/// Processes the `else` clause of an if-else statement, handling both `else if` and `else` blocks.
///
/// # Arguments
///
/// * `db` - The syntax group which provides access to the syntax tree.
/// * `else_clause` - The `ElseClause` AST node representing the else clause.
/// * `indent` - A string representing the indentation to be applied to the else clause.
///
/// # Returns
///
/// A string representing the formatted content of the else clause.
///
/// # Example
///
/// Input:
/// ```cairo
/// else if x > 5 {
///     let y = 10;
///     break;
/// }
/// ```
/// Output:
/// ```cairo
/// else if x > 5 {
///     let y = 10;
/// }
/// ```
///
/// This function formats the `else` or `else if` block and returns it as a string.
pub fn remove_break_from_else_clause<'db>(
    db: &'db dyn SyntaxGroup,
    else_clause: ElseClause<'db>,
    indent: &str,
) -> String {
    let mut else_body = String::new();
    match else_clause.else_block_or_if(db) {
        BlockOrIf::Block(block) => {
            else_body.push_str(&remove_break_from_block(db, block, indent));
        }
        BlockOrIf::If(else_if) => {
            else_body.push_str(&format!(
                "{}else if {} {{\n",
                indent,
                else_if.conditions(db).as_syntax_node().get_text(db)
            ));
            else_body.push_str(&remove_break_from_block(db, else_if.if_block(db), indent));
            else_body.push_str(&format!("{indent}}}\n"));
        }
    }
    else_body
}

/// Inverts a logical condition, swapping `&&` for `||` and vice versa.
///
/// # Arguments
///
/// * `condition` - A string representing the logical condition to invert.
///
/// # Returns
///
/// A string representing the inverted condition.
///
/// # Example
///
/// Input: `"x >= 5 && y < 10"`
/// Output: `"x < 5 || y >= 10"`
///
/// This inverts both the logical operator (`&&` becomes `||`) and the comparison operators.
pub fn invert_condition(condition: &str) -> String {
    if condition.contains("&&") {
        condition
            .split("&&")
            .map(|part| invert_simple_condition(part.trim()))
            .collect::<Vec<_>>()
            .join(" || ")
    } else if condition.contains("||") {
        condition
            .split("||")
            .map(|part| invert_simple_condition(part.trim()))
            .collect::<Vec<_>>()
            .join(" && ")
    } else {
        invert_simple_condition(condition)
    }
}

/// Inverts a simple condition like `>=` to `<`, `==` to `!=`, etc.
///
/// # Arguments
///
/// * `condition` - A string representing a simple condition (e.g., `>=`, `==`, `!=`).
///
/// # Returns
///
/// A string representing the inverted condition.
///
/// # Example
///
/// Input: `"x >= 5"`
/// Output: `"x < 5"`
///
/// This will invert the condition by reversing the comparison operator.
pub fn invert_simple_condition(condition: &str) -> String {
    if condition.contains(">=") {
        condition.replace(">=", "<")
    } else if condition.contains("<=") {
        condition.replace("<=", ">")
    } else if condition.contains(">") {
        condition.replace(">", "<=")
    } else if condition.contains("<") {
        condition.replace("<", ">=")
    } else if condition.contains("==") {
        condition.replace("==", "!=")
    } else if condition.contains("!=") {
        condition.replace("!=", "==")
    } else {
        format!("!({condition})")
    }
}

pub fn indent_snippet(input: &str, initial_indentation: usize) -> String {
    let mut indented_code = String::new();
    let mut indentation_level = initial_indentation;
    let indent = "    "; // 4 spaces for each level of indentation
    let mut lines = input.split('\n').peekable();
    while let Some(line) = lines.next() {
        let trim = line.trim();
        // Decrease indentation level if line starts with a closing brace
        if trim.starts_with('}') && indentation_level > 0 {
            indentation_level -= 1;
        }

        // Add current indentation level to the line
        if !trim.is_empty() {
            indented_code.push_str(&indent.repeat(indentation_level));
        }
        indented_code.push_str(trim);
        if lines.peek().is_some() {
            indented_code.push('\n');
        }
        // Increase indentation level if line ends with an opening brace
        if trim.ends_with('{') {
            indentation_level += 1;
        }
    }

    indented_code
}

/// Checks whether a function call argument represents the literal zero.
///
/// # Arguments
///
/// * `arg` - A reference to an [`ExprFunctionCallArg`] that may contain a literal value.
/// * `arenas` - The arenas holding the expression nodes where the literal is stored.
///
/// # Returns
///
/// Returns `true` if the provided argument is a literal whose value equals 0; otherwise returns `false`.
///
/// # Example
///
/// ```rust,ignore
/// if is_zero(&expr_func.args, arenas) {
///     // do something specific if the first argument is zero
/// }
/// ```
pub fn is_zero(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    matches!(
        arg,
        ExprFunctionCallArg::Value(expr)
            if matches!(&arenas.exprs[*expr], Expr::Literal(val) if val.value == BigInt::from(0))
    )
}

/// Checks whether a function call argument represents the literal one.
///
/// # Arguments
///
/// * `arg` - A reference to an [`ExprFunctionCallArg`] that may contain a literal value.
/// * `arenas` - The arenas holding the expression nodes where the literal is stored.
///
/// # Returns
///
/// Returns `true` if the provided argument is a literal whose value equals 1; otherwise returns `false`.
///
/// # Example
///
/// ```rust,ignore
/// if is_one(&expr_func.args[1], arenas) {
///     // do something specific if the second argument is one
/// }
/// ```
pub fn is_one(arg: &ExprFunctionCallArg, arenas: &Arenas) -> bool {
    matches!(
        arg,
        ExprFunctionCallArg::Value(expr)
            if matches!(&arenas.exprs[*expr], Expr::Literal(val) if val.value == BigInt::from(1))
    )
}

fn check_if_panic_block<'db>(
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    expr_id: ExprId,
) -> bool {
    if_chain! {
        if let Expr::Block(ref panic_block) = arenas.exprs[expr_id];
        if let Some(panic_block_tail) = panic_block.tail;
        if let Expr::FunctionCall(ref expr_func_call) = arenas.exprs[panic_block_tail];
        if expr_func_call.function.full_path(db) == PANIC_WITH_BYTE_ARRAY_PATH;
        then {
            return true;
        }
    }
    false
}

fn check_if_inline_panic<'db>(
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    expr_id: ExprId,
) -> bool {
    if_chain! {
        if let Expr::FunctionCall(ref expr_func_call) = arenas.exprs[expr_id];
        if expr_func_call.function.full_path(db) == PANIC_PATH || expr_func_call.function.full_path(db) == PANIC_WITH_BYTE_ARRAY_PATH;
        then {
            return true;
        }
    }
    false
}

pub fn is_panic_expr<'db>(
    db: &'db dyn SemanticGroup,
    arenas: &Arenas<'db>,
    expr_id: ExprId,
) -> bool {
    check_if_inline_panic(db, arenas, expr_id) || check_if_panic_block(db, arenas, expr_id)
}

pub fn find_module_file_containing_node<'db>(
    db: &'db dyn LinterGroup,
    node: SyntaxNode<'db>,
) -> Option<ModuleFileId<'db>> {
    let module_id = find_module_containing_node(db, node)?;
    let file_index = FileIndex(0);
    Some(ModuleFileId(module_id, file_index))
}

fn find_module_containing_node<'db>(
    db: &'db dyn LinterGroup,
    node: SyntaxNode<'db>,
) -> Option<ModuleId<'db>> {
    // Get the main module of the main file that leads to the node.
    // The node may be located in a virtual file of a submodule.
    // This code attempts to get the absolute "parent" of both "module" and "file" parts.
    let main_module = {
        // Get the file where the node is located.
        // This might be a virtual file generated by a compiler plugin.
        let node_file_id = node.stable_ptr(db).file_id(db);

        // Get the root module of a file containing the node.
        let node_main_module = db.file_modules(node_file_id).ok()?.iter().copied().next()?;

        // Get the main module of the file.
        let main_file = db.module_main_file(node_main_module).ok()?;

        // Get the main module of that file.
        db.file_modules(main_file).ok()?.iter().copied().next()?
    };

    // Get the stack (bottom-up) of submodule names in the file containing the node, in the main
    // module, that lead to the node.
    node.ancestors(db)
        .filter(|node| node.kind(db) == SyntaxKind::ItemModule)
        .map(|node| {
            ast::ItemModule::from_syntax_node(db, node)
                .stable_ptr(db)
                .name_green(db)
                .identifier(db)
        })
        // Buffer the stack to get DoubleEndedIterator.
        .collect::<Vec<_>>()
        .into_iter()
        // And get id of the (sub)module containing the node by traversing this stack top-down.
        .try_rfold(main_module, |module, name| {
            let ModuleItemId::Submodule(submodule) =
                db.module_item_by_name(module, name.into()).ok()??
            else {
                return None;
            };
            Some(ModuleId::Submodule(submodule))
        })
}

pub fn format_fixed_file(
    db: &dyn SyntaxGroup,
    formatter_config: FormatterConfig,
    content: String,
) -> String {
    let virtual_file = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "string_to_format".into(),
        content: content.clone().into(),
        code_mappings: [].into(),
        kind: FileKind::Module,
        original_item_removed: false,
    })
    .intern(db);
    let mut diagnostics = DiagnosticsBuilder::default();
    let syntax_root =
        Parser::parse_file(db, &mut diagnostics, virtual_file, content.as_str()).as_syntax_node();
    get_formatted_file(db, &syntax_root, formatter_config)
}

pub fn is_item_ancestor_of_module<'db>(
    db: &'db dyn SemanticGroup,
    searched_item: &LookupItemId<'db>,
    module_id: ModuleId<'db>,
) -> bool {
    let Ok(items) = module_id.module_data(db) else {
        return false;
    };

    for item in items.items(db).iter() {
        match item {
            ModuleItemId::Submodule(submodule_id) => {
                if is_item_ancestor_of_module(db, searched_item, ModuleId::Submodule(*submodule_id))
                {
                    return true;
                }
            }
            ModuleItemId::Impl(impl_id) => {
                if LookupItemId::ModuleItem(ModuleItemId::Impl(*impl_id)) == *searched_item {
                    return true;
                }

                if let Ok(functions) = db.impl_functions(*impl_id) {
                    for (_, impl_fn_id) in functions.iter() {
                        if LookupItemId::ImplItem(ImplItemId::Function(*impl_fn_id))
                            == *searched_item
                        {
                            return true;
                        }
                    }
                }

                if let Ok(types) = db.impl_types(*impl_id) {
                    for (impl_type_id, _) in types.iter() {
                        if LookupItemId::ImplItem(ImplItemId::Type(*impl_type_id)) == *searched_item
                        {
                            return true;
                        }
                    }
                }

                if let Ok(consts) = db.impl_constants(*impl_id) {
                    for (impl_const_id, _) in consts.iter() {
                        if LookupItemId::ImplItem(ImplItemId::Constant(*impl_const_id))
                            == *searched_item
                        {
                            return true;
                        }
                    }
                }
            }
            ModuleItemId::Trait(trait_id) => {
                if LookupItemId::ModuleItem(ModuleItemId::Trait(*trait_id)) == *searched_item {
                    return true;
                }

                if let Ok(functions) = db.trait_functions(*trait_id) {
                    for (_, trait_fn_id) in functions.iter() {
                        if LookupItemId::TraitItem(TraitItemId::Function(*trait_fn_id))
                            == *searched_item
                        {
                            return true;
                        }
                    }
                }

                if let Ok(types) = db.trait_types(*trait_id) {
                    for (_, trait_type_id) in types.iter() {
                        if LookupItemId::TraitItem(TraitItemId::Type(*trait_type_id))
                            == *searched_item
                        {
                            return true;
                        }
                    }
                }

                if let Ok(consts) = db.trait_constants(*trait_id) {
                    for (_, trait_const_id) in consts.iter() {
                        if LookupItemId::TraitItem(TraitItemId::Constant(*trait_const_id))
                            == *searched_item
                        {
                            return true;
                        }
                    }
                }
            }
            _ => {
                if LookupItemId::ModuleItem(*item) == *searched_item {
                    return true;
                }
            }
        }
    }
    false
}
