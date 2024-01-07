mod actions;
mod commands;

use actions::list::list_duplicates;
use commands::arguments_parser::{Action, Cli};

use clap::Parser;

fn main() {
    let args = Cli::parse();
    perform_action(args);
}

fn perform_action(
    Cli {
        action,
        path,
        include_hidden,
        depth,
    }: Cli,
) {
    match action {
        Action::List => list_duplicates(&path, include_hidden, depth),
        Action::Organize => println!("processing {}", action),
    }
}
