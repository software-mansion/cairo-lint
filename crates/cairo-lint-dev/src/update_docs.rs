use std::{env, fs, path::Path, process::Command};

use cairo_lint_core::{
    context::{find_lint_by_struct_name, Lint},
    lints::single_match::DestructMatch,
};
use if_chain::if_chain;
use minijinja::{context, Environment};
use scarb_ui::{components::Status, Ui};
use serde_json::Value;

static DOC_PATH: &str = "./target/doc/cairo_lint_core.json";

#[derive(Debug)]
pub struct LintDoc {
    name: String,
    docs: Option<String>,
    filename: String,
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
}

fn get_new_html() -> anyhow::Result<String> {
    let template_html = fs::read_to_string("lint_docs_template.html")?;
    let mut env = Environment::new();
    env.add_template("hello", &template_html)?;
    let template = env.get_template("hello")?;
    Ok(template.render(context!(name => "John"))?)
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

    let data = fs::read_to_string(DOC_PATH)?;
    let value: Value = serde_json::from_str(&data)?;
    let items_map = value.get("index");

    if let Some(index) = items_map {
        if let Some(index_map) = index.as_object() {
            let lint_struct_name = value
                .pointer("/inner/impl/for/resolved_path/path")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string();
            return Ok(index_map
                .values()
                .filter(|value| {
                    value
                        .pointer("/inner/impl/trait/path")
                        .map_or(false, |path| path == "Lint")
                })
                .map(|value| LintDoc {
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
                    filename: value
                        .pointer("/span/filename")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                })
                .collect());
        }
    }
    Ok(vec![])
}
