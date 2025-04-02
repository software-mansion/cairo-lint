use std::{collections::BTreeMap, path::PathBuf, sync::Arc};

use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_defs::{db::DefsGroup, ids::ModuleId};
use cairo_lang_diagnostics::Diagnostics;
use cairo_lang_filesystem::{
    db::{init_dev_corelib, CrateConfiguration, Edition, ExperimentalFeaturesConfig, FilesGroup},
    ids::{CrateId, CrateLongId, Directory, FileKind, FileLongId, VirtualFile},
};
use cairo_lang_semantic::{db::SemanticGroup, SemanticDiagnostic};
use cairo_lang_utils::{ordered_hash_map::OrderedHashMap, LookupIntern};
use cairo_lint::{context::get_unique_allowed_names, CairoLintToolMetadata};
use scarb::find_scarb_managed_core;

mod scarb;

pub fn get_diags(crate_id: CrateId, db: &mut RootDatabase) -> Vec<Diagnostics<SemanticDiagnostic>> {
    if let Ok(path) = std::env::var("CORELIB_PATH") {
        init_dev_corelib(db, PathBuf::from(path));
    } else if let Some(path) = find_scarb_managed_core() {
        init_dev_corelib(db, path);
    } else {
        panic!("Missing corelib path. CORELIB_PATH env or Scarb managed corelib is required.");
    }
    let mut diagnostics = Vec::new();
    let module_file = db.module_main_file(ModuleId::CrateRoot(crate_id)).unwrap();
    if db.file_content(module_file).is_none() {
        match module_file.lookup_intern(db) {
            FileLongId::OnDisk(_path) => {}
            FileLongId::Virtual(_) => panic!("Missing virtual file."),
            FileLongId::External(_) => (),
        }
    }

    for module_id in &*db.crate_modules(crate_id) {
        diagnostics.push(db.module_semantic_diagnostics(*module_id).unwrap());
    }
    diagnostics
}

pub fn get_cairo_lint_tool_metadata_with_all_lints_enabled() -> CairoLintToolMetadata {
    let names = get_unique_allowed_names();
    names
        .into_iter()
        .map(|name| (name.to_string(), true))
        .collect()
}

#[macro_export]
macro_rules! test_lint_fixer {
  ($before:literal, @$expected_fix:literal) => {{
    let expected_value: &str = $before;
    test_lint_fixer!(expected_value, @$expected_fix, false);
  }};
  ($before:ident, @$expected_fix:literal) => {
    test_lint_fixer!($before, @$expected_fix, false)
  };
  ($before:literal, @$expected_fix:literal, $is_nested:literal) => {{
    let expected_value: &str = $before;
    test_lint_fixer!(expected_value, @$expected_fix, $is_nested)
  }};
  ($before:ident, @$expected_fix:literal, $is_nested:literal) => {{
    use ::cairo_lang_utils::Upcast;
    let mut code = String::from($before);
    let mut testing_suite = ::cairo_lang_semantic::plugin::PluginSuite::default();
    testing_suite.add_analyzer_plugin_ex(::std::sync::Arc::new(::cairo_lint::plugin::CairoLint::new(true, $crate::helpers::get_cairo_lint_tool_metadata_with_all_lints_enabled())));
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_default_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_default_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_default_plugin_suite(testing_suite)
      .build()
      .unwrap();
    let diags = $crate::helpers::get_diags(
      crate::helpers::setup_test_crate_ex(&mut db, $before),
      &mut db,
    );
    let semantic_diags: Vec<_> = diags
      .clone()
      .into_iter()
      .flat_map(|diag| diag.get_all())
      .collect();
    let unused_imports: ::std::collections::HashMap<::cairo_lang_filesystem::ids::FileId, ::std::collections::HashMap<::cairo_lang_syntax::node::SyntaxNode, ::cairo_lint::fixes::ImportFix>> =
      ::cairo_lint::fixes::collect_unused_imports(&db, &semantic_diags);
    let mut fixes = if unused_imports.keys().len() > 0 {
      let current_file_id = unused_imports.keys().next().unwrap();
      ::cairo_lint::fixes::apply_import_fixes(&db, unused_imports.get(&current_file_id).unwrap())
    } else {
      Vec::new()
    };
    for diag in diags.iter().flat_map(|diags| diags.get_all()) {
      if !matches!(diag.kind, ::cairo_lang_semantic::diagnostic::SemanticDiagnosticKind::UnusedImport(_)) {
        if let Some((fix_node, fix)) = ::cairo_lint::fixes::fix_semantic_diagnostic(&db, &diag) {
          let span = fix_node.span(db.upcast());
          fixes.push(::cairo_lint::fixes::Fix {
            span,
            suggestion: fix,
          });
        }
      }
    }
    fixes.sort_by_key(|v| std::cmp::Reverse(v.span.start));
    if !$is_nested {
      for fix in fixes.iter() {
        code.replace_range(fix.span.to_str_range(), &fix.suggestion);
      }
    } else {
      code = "Contains nested diagnostics can't fix it".to_string();
    }
      ::insta::assert_snapshot!(code, @$expected_fix);
  }};
}

#[macro_export]
macro_rules! test_lint_diagnostics {
  ($before:literal, @$expected_diagnostics:literal) => {{
    let expected_value: &str = $before;
    test_lint_diagnostics!(expected_value, @$expected_diagnostics)
  }};
  ($before:ident, @$expected_diagnostics:literal) => {{
    let mut testing_suite = ::cairo_lang_semantic::plugin::PluginSuite::default();
    testing_suite.add_analyzer_plugin_ex(::std::sync::Arc::new(::cairo_lint::plugin::CairoLint::new(true, $crate::helpers::get_cairo_lint_tool_metadata_with_all_lints_enabled())));
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_default_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_default_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_default_plugin_suite(testing_suite)
      .build()
      .unwrap();
    let diags = $crate::helpers::get_diags(
      crate::helpers::setup_test_crate_ex(&mut db, $before),
      &mut db,
    );
    let formatted_diags = diags
      .into_iter()
      .flat_map(|diags| {
        diags
          .get_all()
          .iter()
          .map(|diag| ::cairo_lint::diagnostics::format_diagnostic(diag, &db))
          .collect::<Vec<_>>()
      })
      .collect::<String>()
      .trim()
      .to_string();
      ::insta::assert_snapshot!(formatted_diags, @$expected_diagnostics);
  }};
}

use cairo_lang_filesystem::db::CrateSettings;
use cairo_lang_utils::Intern;

use crate::CRATE_CONFIG;

pub fn setup_test_crate_ex(db: &mut dyn SemanticGroup, content: &str) -> CrateId {
    let file_id = FileLongId::Virtual(VirtualFile {
        parent: None,
        name: "lib.cairo".into(),
        content: content.into(),
        code_mappings: [].into(),
        kind: FileKind::Module,
    })
    .intern(db);

    let settings = CrateSettings {
        name: None,
        edition: Edition::latest(),
        version: None,
        dependencies: Default::default(),
        experimental_features: ExperimentalFeaturesConfig {
            negative_impls: true,
            associated_item_constraints: true,
            coupons: true,
        },
        cfg_set: Default::default(),
    };
    let crate_config = CrateConfiguration {
        root: Directory::Virtual {
            files: BTreeMap::from([("lib.cairo".into(), file_id)]),
            dirs: Default::default(),
        },
        settings,
        cache_file: None,
    };

    let crate_id = CrateLongId::Virtual {
        name: "test".into(),
        file_id,
        settings: CRATE_CONFIG.to_string(),
        cache_file: None,
    }
    .intern(db);
    // TODO: (wawel37) make it in proper way
    db.set_crate_configs(Arc::new(OrderedHashMap::from([(crate_id, crate_config)])));

    crate_id
}
