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
use cairo_lang_defs::db::DefsGroup;
use cairo_lang_defs::ids::{FileIndex, ModuleFileId, ModuleId, ModuleItemId};
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::{Arenas, Expr, ExprFunctionCallArg};
use cairo_lang_syntax::node::ast::{BlockOrIf, ElseClause, ExprBlock, Statement};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::helpers::GetIdentifier;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{ast, SyntaxNode, TypedSyntaxNode};
use cairo_lang_utils::Upcast;
use num_bigint::BigInt;

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
pub fn remove_break_from_block(db: &dyn SyntaxGroup, block: ExprBlock, indent: &str) -> String {
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
pub fn remove_break_from_else_clause(
    db: &dyn SyntaxGroup,
    else_clause: ElseClause,
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
                else_if.condition(db).as_syntax_node().get_text(db)
            ));
            else_body.push_str(&remove_break_from_block(db, else_if.if_block(db), indent));
            else_body.push_str(&format!("{}}}\n", indent));
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
        format!("!({})", condition)
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

pub fn find_module_file_containing_node(
    db: &dyn SemanticGroup,
    node: &SyntaxNode,
) -> Option<ModuleFileId> {
    let module_id = find_module_containing_node(db, node)?;
    let file_index = FileIndex(0);
    Some(ModuleFileId(module_id, file_index))
}

fn find_module_containing_node(db: &dyn SemanticGroup, node: &SyntaxNode) -> Option<ModuleId> {
    let syntax_db = db.upcast();
    // let syntax_db = syntax_db.upcast();

    // Get the main module of the main file that leads to the node.
    // The node may be located in a virtual file of a submodule.
    // This code attempts to get the absolute "parent" of both "module" and "file" parts.
    let main_module = {
        // Get the file where the node is located.
        // This might be a virtual file generated by a compiler plugin.
        let node_file_id = node.stable_ptr().file_id(syntax_db);

        // Get the root module of a file containing the node.
        let node_main_module = db.file_modules(node_file_id).ok()?.iter().copied().next()?;

        // // Get the main module of the file.
        let main_file = db.module_main_file(node_main_module).ok()?;

        // Get the main module of that file.
        db.file_modules(node_file_id).ok()?.iter().copied().next()?
    };

    // Get the stack (bottom-up) of submodule names in the file containing the node, in the main
    // module, that lead to the node.
    node.ancestors()
        .filter(|node| node.kind(syntax_db) == SyntaxKind::ItemModule)
        .map(|node| {
            ast::ItemModule::from_syntax_node(syntax_db, node)
                .stable_ptr()
                .name_green(syntax_db)
                .identifier(syntax_db)
        })
        // Buffer the stack to get DoubleEndedIterator.
        .collect::<Vec<_>>()
        .into_iter()
        // And get id of the (sub)module containing the node by traversing this stack top-down.
        .try_rfold(main_module, |module, name| {
            let ModuleItemId::Submodule(submodule) =
                db.module_item_by_name(module, name).ok()??
            else {
                return None;
            };
            Some(ModuleId::Submodule(submodule))
        })
}
