mod commands;
use clap::Parser;
use commands::arguments_parser::{Action, Cli};

fn perform_action(aciton: Action) {
    match aciton {
        Action::List => println!("processing {}", aciton),
        Action::Organize => println!("processing {}", aciton),
    }
}

fn main() {
    let args = Cli::parse();
    perform_action(args.action);
}
