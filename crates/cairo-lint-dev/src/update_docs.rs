//! Main part of this module is the rustdoc. It is used to generate the documentation for the lints.
//! Currently the `rustdoc 1.84.1 (e71f9a9a9 2025-01-27)` is being used here.
//! Please update the version here if you plan to use later versions.

use std::{env, fs, process::Command};

use cairo_lint_core::context::find_lint_by_struct_name;
use scarb_ui::{components::Status, Ui};
use serde::Serialize;
use serde_json::Value;

static RUSTDOC_PATH: &str = "target/doc/cairo_lint_core.json";
static LINT_METADATA_OUTPUT_PATH: &str = "website/lints_metadata.json";
static LINT_REPO_BASE_URL: &str = "https://github.com/software-mansion/cairo-lint/tree/main/";
static LINT_DOCS_BASE_PATH: &str = "website/docs/lints/";

#[derive(Debug, Serialize)]
pub struct LintDoc {
    name: String,
    docs: Option<String>,
    source_link: String,
}

impl LintDoc {
    pub fn from_rustdoc_json_item(value: &Value) -> Self {
        let lint_struct_name = value
            .pointer("/inner/impl/for/resolved_path/path")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let filename = value
            .pointer("/span/filename")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        let struct_start_line = value.pointer("/span/begin/0").unwrap().as_u64().unwrap();
        LintDoc {
            name: find_lint_by_struct_name(&lint_struct_name)
                .expect(&format!(
                    "Could not find the lint {} inside the Lint Context.",
                    lint_struct_name
                ))
                .allowed_name()
                .to_string(),
            docs: value.get("docs").and_then(|doc| {
                if doc.is_null() {
                    None
                } else {
                    Some(doc.as_str().unwrap().to_string())
                }
            }),
            source_link: format!("{}{}#L{}", LINT_REPO_BASE_URL, filename, struct_start_line),
        }
    }
}

pub fn update_docs(ui: &Ui) {
    let docs = match get_docs_as_json() {
        Ok(docs) => docs,
        Err(e) => {
            ui.print(Status::new(
                "error",
                &format!("Failed to get docs as json: {:?}", e),
            ));
            return;
        }
    };

    // Write the docs to the lints_metadata.json file inside the website directory.
    match fs::write(
        LINT_METADATA_OUTPUT_PATH,
        serde_json::to_string_pretty(&docs).unwrap(),
    ) {
        Ok(_) => ui.print(Status::new(
            "info",
            &format!(
                "Docs metadata successfully written to {}",
                LINT_METADATA_OUTPUT_PATH
            ),
        )),
        Err(e) => {
            ui.print(Status::new(
                "error",
                &format!(
                    "Failed to write docs to {}: {:?}",
                    LINT_METADATA_OUTPUT_PATH, e
                ),
            ));
            return;
        }
    };

    // Write docs content inside the markdown file inside the website docs directory.
    for doc in docs.iter() {
        let doc_path = format!("{}{}.md", LINT_DOCS_BASE_PATH, doc.name);
        let doc_content = doc.docs.clone().unwrap_or(String::new());
        fs::write(
            &doc_path,
            format!(
                "# {}\n\n[Source Code]({})\n\n{}",
                doc.name, doc.source_link, doc_content
            ),
        )
        .unwrap();
        ui.print(Status::new(
            "info",
            &format!("Docs successfully written to {}", doc_path),
        ))
    }
}

fn get_docs_as_json() -> anyhow::Result<Vec<LintDoc>> {
    let workspace_root = env::current_dir().unwrap();

    let output = Command::new("cargo")
        .arg("+nightly")
        .arg("rustdoc")
        .arg("--output-format")
        .arg("json")
        .arg("-Z")
        .arg("unstable-options")
        .arg("-p")
        .arg("cairo-lint-core")
        .current_dir(&workspace_root)
        .output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "Failed to run cargo rustdoc: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let data = fs::read_to_string(RUSTDOC_PATH)?;
    let value: Value = serde_json::from_str(&data)?;
    let items_map = value.get("index");

    if let Some(index) = items_map {
        if let Some(index_map) = index.as_object() {
            return Ok(index_map
                .values()
                .filter(|value| {
                    value
                        .pointer("/inner/impl/trait/path")
                        .map_or(false, |path| path == "Lint")
                })
                .map(LintDoc::from_rustdoc_json_item)
                .collect());
        }
    }
    Ok(vec![])
}
