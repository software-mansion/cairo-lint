use clap::{Parser, Subcommand};
use scarb_ui::{OutputFormat, Ui};

mod update_docs;

fn main() {
    let dev = Dev::parse();
    let ui = Ui::new(Default::default(), OutputFormat::Text);

    match dev.command {
        DevCommand::UpdateDocs => {
            update_docs::update_docs(&ui);
        }
    }
}

#[derive(Parser)]
#[command(name = "dev", about)]
struct Dev {
    #[command(subcommand)]
    command: DevCommand,
}

#[derive(Subcommand)]
enum DevCommand {
    #[command(name = "update_docs")]
    UpdateDocs,
}
