use anyhow::Result;
use cairo_lint::context::find_lint_by_struct_name;
use clap::Parser;
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Serializer, Value};
use std::{env, fs, process::Command};

static RUSTDOC_PATH: &str = "target/doc/cairo_lint.json";
static LINT_METADATA_OUTPUT_PATH: &str = "website/lints_metadata.json";
static DEFAULT_PROFILE_OUTPUT_PATH: &str = "website/docs/default_profile.md";
static LINT_REPO_BASE_URL: &str = "https://github.com/software-mansion/cairo-lint/tree/main/";
static LINT_DOCS_RELATIVE_PATH: &str = "lints/";
static LINT_DOCS_BASE_PATH: &str = "website/docs/lints/";

#[derive(Debug, Serialize)]
struct LintDoc {
    name: String,
    docs: Option<String>,
    enabled: bool,
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
        let lint = find_lint_by_struct_name(&lint_struct_name).unwrap_or_else(|| {
            panic!("Could not find the lint inside the Lint Context: {lint_struct_name}")
        });
        LintDoc {
            name: lint.allowed_name().to_string(),
            docs: value.get("docs").and_then(|doc| {
                if doc.is_null() {
                    None
                } else {
                    Some(doc.as_str().unwrap().to_string())
                }
            }),
            enabled: lint.is_enabled(),
            source_link: format!("{LINT_REPO_BASE_URL}{filename}#L{struct_start_line}"),
        }
    }
}

#[derive(Parser)]
pub struct Args;

pub fn main(_: Args) -> Result<()> {
    let docs = match get_docs_as_json() {
        Ok(docs) => docs,
        Err(e) => {
            eprintln!("Failed to get docs as json: {e:?}");
            return Err(e);
        }
    };

    let mut buf: Vec<u8> = Vec::new();
    let formatter = PrettyFormatter::with_indent(b"    ");
    let mut serializer = Serializer::with_formatter(&mut buf, formatter);
    docs.serialize(&mut serializer).unwrap();

    // Write the docs to the lints_metadata.json file inside the website directory.
    match fs::write(
        LINT_METADATA_OUTPUT_PATH,
        String::from_utf8(buf).unwrap() + "\n",
    ) {
        Ok(_) => println!("Docs metadata successfully written to {LINT_METADATA_OUTPUT_PATH}"),
        Err(e) => {
            eprintln!("Failed to write docs to {LINT_METADATA_OUTPUT_PATH}: {e:?}");
            return Err(e.into());
        }
    };

    let disabled_lints = docs.iter().filter(|doc| !doc.enabled);
    let disabled_lints_list = disabled_lints
        .map(|doc| {
            format!(
                "-   [{}]({}{}.md)\n",
                doc.name, LINT_DOCS_RELATIVE_PATH, doc.name
            )
        })
        .collect::<String>();
    fs::write(DEFAULT_PROFILE_OUTPUT_PATH, format!("# Default Profile\n\nBy default, all lint rules are **enabled** with the exception of:\n\n{disabled_lints_list}")).unwrap();

    // Write docs content inside the markdown file inside the website docs directory.
    for doc in docs.iter() {
        let doc_path = format!("{}{}.md", LINT_DOCS_BASE_PATH, doc.name);
        let doc_content = doc.docs.clone().unwrap_or(String::new());
        let enabled_text = if doc.enabled { "Enabled" } else { "Disabled" };
        fs::write(
            &doc_path,
            format!(
                "# {}\n\nDefault: **{enabled_text}**\n\n[Source Code]({})\n\n{}\n",
                doc.name, doc.source_link, doc_content
            ),
        )
        .unwrap();
        println!("Docs successfully written to {doc_path}");
    }

    Ok(())
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
        .arg("cairo-lint")
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
                        .is_some_and(|path| path == "Lint")
                })
                .map(LintDoc::from_rustdoc_json_item)
                .collect());
        }
    }
    Ok(vec![])
}
