use cairo_lang_defs::ids::{
    ConstantLongId, EnumLongId, ExternFunctionLongId, ExternTypeLongId, FreeFunctionLongId,
    ImplAliasLongId, ImplDefLongId, LanguageElementId, MacroDeclarationLongId, ModuleId,
    ModuleItemId, ModuleTypeAliasLongId, StructLongId, TraitLongId, UseLongId,
};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_filesystem::ids::{FileId, FileLongId};
use cairo_lang_syntax::node::helpers::QueryAttrs;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::{SyntaxNode, TypedSyntaxNode, ast};
use cairo_lang_utils::Intern;
use if_chain::if_chain;
use std::collections::HashSet;

use crate::context::{
    get_all_checking_functions, get_name_for_diagnostic_message, is_lint_enabled_by_default,
};
use crate::{CairoLintToolMetadata, CorelibContext};

use crate::mappings::{get_origin_module_item_as_syntax_node, get_origin_syntax_node};

mod db;
use cairo_lang_defs::db::{DefsGroup, get_all_path_leaves};
pub use db::{LinterAnalysisDatabase, LinterAnalysisDatabaseBuilder};
use salsa::Database;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct LinterDiagnosticParams {
    pub only_generated_files: bool,
    pub tool_metadata: CairoLintToolMetadata,
}

pub trait LinterGroup: Database {
    fn linter_diagnostics<'db>(
        &'db self,
        params: LinterDiagnosticParams,
        module_id: ModuleId<'db>,
    ) -> &'db Vec<PluginDiagnostic<'db>> {
        linter_diagnostics(self.as_dyn_database(), params, module_id)
    }

    fn corelib_context<'db>(&'db self) -> &'db CorelibContext<'db> {
        corelib_context(self.as_dyn_database())
    }
}

impl<T: Database + ?Sized> LinterGroup for T {}

#[tracing::instrument(skip_all, level = "trace")]
#[salsa::tracked(returns(ref))]
fn linter_diagnostics<'db>(
    db: &'db dyn Database,
    params: LinterDiagnosticParams,
    module_id: ModuleId<'db>,
) -> Vec<PluginDiagnostic<'db>> {
    let mut diags: Vec<(PluginDiagnostic, FileId)> = Vec::new();
    let Ok(module_data) = module_id.module_data(db) else {
        return Vec::default();
    };

    let mut linted_nodes: HashSet<SyntaxNode> = HashSet::new();

    for item in module_data.items(db) {
        let mut item_diagnostics = Vec::new();
        let module_file = db.module_main_file(module_id).unwrap();
        let item_file = item.stable_location(db).file_id(db).long(db);
        let is_generated_item =
            matches!(item_file, FileLongId::Virtual(_) | FileLongId::External(_));

        if is_generated_item && !params.only_generated_files {
            let item_syntax_node = item.stable_location(db).stable_ptr().lookup(db);
            let origin_node = get_origin_module_item_as_syntax_node(db, item);

            if_chain! {
                if let Some(node) = origin_node;
                // We want. to make sure that the corresponding item to the origin node was not yet linted. We don't want to duplicate the diagnostics for the same user code.
                if !linted_nodes.contains(&node);
                // We don't do the `==` check here, as the origin node always has the proc macro attributes.
                // It also means that if the macro changed anything in the original item code,
                // we won't be processing it, as it might lead to unexpected behavior.
                if node.get_text_without_trivia(db).long(db).as_str().contains(item_syntax_node.get_text_without_trivia(db).long(db).as_str());
                then {
                    let checking_functions = get_all_checking_functions();
                    for checking_function in checking_functions {
                        checking_function(db, item, &mut item_diagnostics);
                    }

                    linted_nodes.insert(node);

                    diags.extend(item_diagnostics.into_iter().filter_map(|mut diag| {
                      let ptr = diag.stable_ptr;
                      diag.stable_ptr = get_origin_syntax_node(db, &ptr)?.stable_ptr(db);
                      Some((diag, module_file))}));
                }
            }
        } else if !is_generated_item || params.only_generated_files {
            let checking_functions = get_all_checking_functions();
            for checking_function in checking_functions {
                checking_function(db, item, &mut item_diagnostics);
            }

            diags.extend(item_diagnostics.into_iter().filter_map(|diag| {
                // If the diagnostic is not mapped to an on-disk file, it mean that it's an inline macro diagnostic.
                get_origin_syntax_node(db, &diag.stable_ptr).map(|_| (diag, module_file))
            }));
        }
    }

    diags
        .into_iter()
        .filter(|diag: &(PluginDiagnostic, FileId)| {
            let diagnostic = &diag.0;
            let node = diagnostic.stable_ptr.lookup(db);
            let allowed_name = get_name_for_diagnostic_message(&diagnostic.message).unwrap();
            let default_allowed = is_lint_enabled_by_default(&diagnostic.message).unwrap();
            let is_rule_allowed_globally = *params
                .tool_metadata
                .get(allowed_name)
                .unwrap_or(&default_allowed);
            !node_has_ascendants_with_allow_name_attr(db, node, allowed_name)
                && is_rule_allowed_globally
        })
        .map(|diag| diag.0)
        .collect()
}

#[salsa::tracked(returns(ref))]
fn corelib_context<'db>(db: &'db dyn Database) -> CorelibContext<'db> {
    CorelibContext::new(db)
}

#[tracing::instrument(skip_all, level = "trace")]
fn node_has_ascendants_with_allow_name_attr<'db>(
    db: &'db dyn Database,
    node: SyntaxNode<'db>,
    allowed_name: &'static str,
) -> bool {
    for node in node.ancestors_with_self(db) {
        if node.has_attr_with_arg(db, "allow", allowed_name) {
            return true;
        }
    }
    false
}

/// If the ast node is a ModuleItem, return corresponding ids. Otherwise, returns `None`.
fn module_item_ids_from_ast<'db>(
    db: &'db dyn Database,
    module_id: ModuleId<'db>,
    node: SyntaxNode<'db>,
) -> Option<Vec<ModuleItemId<'db>>> {
    let syntax_db = db;

    Some(match node.kind(syntax_db) {
        SyntaxKind::ItemConstant => vec![ModuleItemId::Constant(
            ConstantLongId(
                module_id,
                ast::ItemConstant::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::FunctionWithBody => vec![ModuleItemId::FreeFunction(
            FreeFunctionLongId(
                module_id,
                ast::FunctionWithBody::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemExternFunction => vec![ModuleItemId::ExternFunction(
            ExternFunctionLongId(
                module_id,
                ast::ItemExternFunction::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemExternType => vec![ModuleItemId::ExternType(
            ExternTypeLongId(
                module_id,
                ast::ItemExternType::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemTrait => vec![ModuleItemId::Trait(
            TraitLongId(
                module_id,
                ast::ItemTrait::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemImpl => {
            vec![ModuleItemId::Impl(
                ImplDefLongId(
                    module_id,
                    ast::ItemImpl::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            )]
        }
        SyntaxKind::ItemStruct => {
            vec![ModuleItemId::Struct(
                StructLongId(
                    module_id,
                    ast::ItemStruct::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            )]
        }
        SyntaxKind::ItemEnum => {
            vec![ModuleItemId::Enum(
                EnumLongId(
                    module_id,
                    ast::ItemEnum::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            )]
        }
        SyntaxKind::ItemUse => {
            let item_use = ast::ItemUse::from_syntax_node(db, node);
            get_all_path_leaves(db, &item_use)
                .into_iter()
                .map(|leaf| {
                    let use_long_id = UseLongId(module_id, leaf.stable_ptr(syntax_db));
                    ModuleItemId::Use(use_long_id.intern(db))
                })
                .collect()
        }
        SyntaxKind::ItemTypeAlias => vec![ModuleItemId::TypeAlias(
            ModuleTypeAliasLongId(
                module_id,
                ast::ItemTypeAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemImplAlias => vec![ModuleItemId::ImplAlias(
            ImplAliasLongId(
                module_id,
                ast::ItemImplAlias::from_syntax_node(db, node).stable_ptr(db),
            )
            .intern(db),
        )],
        SyntaxKind::ItemMacroDeclaration => {
            vec![ModuleItemId::MacroDeclaration(
                MacroDeclarationLongId(
                    module_id,
                    ast::ItemMacroDeclaration::from_syntax_node(db, node).stable_ptr(db),
                )
                .intern(db),
            )]
        }
        _ => return None,
    })
}
