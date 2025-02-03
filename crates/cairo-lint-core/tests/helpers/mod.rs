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
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_plugin_suite(::cairo_lint_core::plugin::cairo_lint_plugin_suite())
      .build()
      .unwrap();
    let diags = ::cairo_lint_test_utils::get_diags(
      ::cairo_lang_semantic::test_utils::setup_test_crate_ex(db.upcast(), $before, Some($crate::CRATE_CONFIG)),
      &mut db,
    );
    let semantic_diags: Vec<_> = diags
      .clone()
      .into_iter()
      .flat_map(|diag| diag.get_all())
      .collect();
    let unused_imports: ::std::collections::HashMap<::cairo_lang_filesystem::ids::FileId, ::std::collections::HashMap<::cairo_lang_syntax::node::SyntaxNode, ::cairo_lint_core::fix::ImportFix>> =
      ::cairo_lint_core::fix::collect_unused_imports(&db, &semantic_diags);
    let mut fixes = if unused_imports.keys().len() > 0 {
      let current_file_id = unused_imports.keys().next().unwrap();
      ::cairo_lint_core::fix::apply_import_fixes(&db, unused_imports.get(&current_file_id).unwrap())
    } else {
      Vec::new()
    };
    for diag in diags.iter().flat_map(|diags| diags.get_all()) {
      if !matches!(diag.kind, ::cairo_lang_semantic::diagnostic::SemanticDiagnosticKind::UnusedImport(_)) {
        if let Some((fix_node, fix)) = ::cairo_lint_core::fix::fix_semantic_diagnostic(&db, &diag) {
          let span = fix_node.span(db.upcast());
          fixes.push(::cairo_lint_core::fix::Fix {
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
    let mut db = ::cairo_lang_compiler::db::RootDatabase::builder()
      .with_plugin_suite(::cairo_lang_semantic::inline_macros::get_default_plugin_suite())
      .with_plugin_suite(::cairo_lang_test_plugin::test_plugin_suite())
      .with_plugin_suite(::cairo_lint_core::plugin::cairo_lint_plugin_suite())
      .build()
      .unwrap();
    let diags = ::cairo_lint_test_utils::get_diags(
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
