use clap::{ArgAction, Parser, ValueEnum};
use core::fmt;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
pub struct Cli {
    #[arg(index = 1, value_enum, default_value = "list")]
    pub action: Action,

    #[arg(
        short,
        long,
        default_value = ".",
        help = "default is the current directory"
    )]
    pub path: PathBuf,

    #[arg(short, long, action=ArgAction::SetTrue)]
    pub include_hidden: bool,

    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Action {
    List,
    Organize,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::List => write!(f, "list"),
            Action::Organize => write!(f, "organize"),
        }
    }
}
