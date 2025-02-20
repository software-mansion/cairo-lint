use std::{env, fs, path::Path, process::Command};

use minijinja::{context, Environment};
use scarb_ui::{components::Status, Ui};
use serde_json::Value;

static DOC_PATH: &str = "/target/doc/cairo_lint_core.json";

pub fn update_docs(ui: &Ui) {
    ui.print(Status::new("test", "Done"));

    // let docs = get_docs_as_json(ui);
    ui.print(Status::new("ejjj", get_new_html().unwrap().as_str()));
}

fn get_new_html() -> anyhow::Result<String> {
    let template_html = fs::read_to_string("lint_docs_template.html")?;
    let mut env = Environment::new();
    env.add_template("hello", &template_html)?;
    let template = env.get_template("hello")?;
    Ok(template.render(context!(name => "John"))?)
}

fn get_docs_as_json() -> anyhow::Result<String> {
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
        .current_dir(workspace_root)
        .output()?;

    // We take only the stderr as rustdoc outputs statuses to stderr also.
    let output_str = output.stderr;

    let data = fs::read_to_string(DOC_PATH)?;
    let v: Value = serde_json::from_str(&data)?;
    return Ok("".to_string());
}
