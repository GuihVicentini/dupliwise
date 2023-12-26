use clap::{Parser, ValueEnum};
use core::fmt;

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
