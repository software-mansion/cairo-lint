use std::path::PathBuf;

use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_defs::{db::DefsGroup, ids::ModuleId};
use cairo_lang_diagnostics::Diagnostics;
use cairo_lang_filesystem::{
    db::{init_dev_corelib, FilesGroup},
    ids::{CrateId, FileLongId},
};
use cairo_lang_semantic::{db::SemanticGroup, SemanticDiagnostic};
use cairo_lang_utils::LookupIntern;

mod scarb;

pub fn get_diags(crate_id: CrateId, db: &mut RootDatabase) -> Vec<Diagnostics<SemanticDiagnostic>> {
    init_dev_corelib(db, PathBuf::from(std::env::var("CORELIB_PATH").unwrap()));
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

/// Try to find a Scarb-managed `core` package if we have Scarb toolchain.
///
/// The easiest way to do this is to create an empty Scarb package and run `scarb metadata` on it.
/// The `core` package will be a component of this empty package.
/// For minimal packages, `scarb metadata` should be pretty fast.
fn find_scarb_managed_core(scarb: &ScarbToolchain) -> Option<UnmanagedCore> {
    let lookup = || {
        let workspace = tempdir()
            .context("failed to create temporary directory")
            .inspect_err(|e| warn!("{e:?}"))
            .ok()?;

        let scarb_toml = workspace.path().join(SCARB_TOML);
        fs::write(
            &scarb_toml,
            indoc! {r#"
                [package]
                name = "cairols_unmanaged_core_lookup"
                version = "1.0.0"
            "#},
        )
        .context("failed to write Scarb.toml")
        .inspect_err(|e| warn!("{e:?}"))
        .ok()?;

        let metadata = scarb
            .silent()
            .metadata(&scarb_toml)
            .inspect_err(|e| warn!("{e:?}"))
            .ok()?;

        // Ensure the workspace directory is deleted after running Scarb.
        // We are ignoring the error, leaving doing proper clean-up to the OS.
        let _ = workspace
            .close()
            .context("failed to wipe temporary directory")
            .inspect_err(|e| warn!("{e:?}"));

        // Scarb is expected to generate only one compilation unit (for our stub package)
        // that will consist of this package and the `core` crate.
        // Therefore, we allow ourselves to liberally just look for any first usage of a package
        // named `core` in all compilation units components we got.
        let path = metadata
            .compilation_units
            .into_iter()
            .find_map(|compilation_unit| {
                compilation_unit
                    .components
                    .iter()
                    .find(|component| component.name == CORELIB_CRATE_NAME)
                    .map(|component| component.source_root().to_path_buf().into_std_path_buf())
            })?;
        let version = metadata
            .packages
            .into_iter()
            .find(|package| package.name == CORELIB_CRATE_NAME)
            .map(|package| package.version)?;

        Some(UnmanagedCore { path, version })
    };

    static CACHE: OnceLock<Option<UnmanagedCore>> = OnceLock::new();
    CACHE.get_or_init(lookup).clone()
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
    testing_suite.add_analyzer_plugin_ex(::std::sync::Arc::new(::cairo_lint_core::plugin::CairoLint::new(true)));
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_plugin_suite(testing_suite)
      .build()
      .unwrap();
    let diags = $crate::helpers::get_diags(
      ::cairo_lang_semantic::test_utils::setup_test_crate_ex(db.upcast(), $before, Some($crate::CRATE_CONFIG)),
      &mut db,
    );
    let semantic_diags: Vec<_> = diags
      .clone()
      .into_iter()
      .flat_map(|diag| diag.get_all())
      .collect();
    let unused_imports: ::std::collections::HashMap<::cairo_lang_filesystem::ids::FileId, ::std::collections::HashMap<::cairo_lang_syntax::node::SyntaxNode, ::cairo_lint_core::fixes::ImportFix>> =
      ::cairo_lint_core::fixes::collect_unused_imports(&db, &semantic_diags);
    let mut fixes = if unused_imports.keys().len() > 0 {
      let current_file_id = unused_imports.keys().next().unwrap();
      ::cairo_lint_core::fixes::apply_import_fixes(&db, unused_imports.get(&current_file_id).unwrap())
    } else {
      Vec::new()
    };
    for diag in diags.iter().flat_map(|diags| diags.get_all()) {
      if !matches!(diag.kind, ::cairo_lang_semantic::diagnostic::SemanticDiagnosticKind::UnusedImport(_)) {
        if let Some((fix_node, fix)) = ::cairo_lint_core::fixes::fix_semantic_diagnostic(&db, &diag) {
          let span = fix_node.span(db.upcast());
          fixes.push(::cairo_lint_core::fixes::Fix {
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
    use ::cairo_lang_utils::Upcast;
    let mut testing_suite = ::cairo_lang_semantic::plugin::PluginSuite::default();
    testing_suite.add_analyzer_plugin_ex(::std::sync::Arc::new(::cairo_lint_core::plugin::CairoLint::new(true)));
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_plugin_suite(testing_suite)
      .build()
      .unwrap();
    let diags = $crate::helpers::get_diags(
      ::cairo_lang_semantic::test_utils::setup_test_crate_ex(db.upcast(), $before, Some($crate::CRATE_CONFIG)),
      &mut db,
    );
    let renderer = ::annotate_snippets::Renderer::plain();
    let formatted_diags = diags
      .into_iter()
      .flat_map(|diags| {
        diags
          .get_all()
          .iter()
          .map(|diag| ::cairo_lint_core::diagnostics::format_diagnostic(diag, &db, &renderer))
          .collect::<Vec<_>>()
      })
      .collect::<String>()
      .trim()
      .to_string();
      ::insta::assert_snapshot!(formatted_diags, @$expected_diagnostics);
  }};
}
