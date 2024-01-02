mod actions;
mod commands;

use actions::list;
use commands::arguments_parser::{Action, Cli};

use clap::Parser;

fn main() {
    let args = Cli::parse();
    perform_action(&args);
}

fn perform_action(
    Cli {
        action,
        dir_path,
        include_hidden,
        recursive,
    }: &Cli,
) {
    match action {
        Action::List => list::list_duplicates(&dir_path, &include_hidden, !recursive),
        Action::Organize => println!("processing {}", action),
    }
}
