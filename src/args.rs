use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TodoArgs {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add a task to the tasklist
    Add(AddCommand),
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The description of the task you're adding to the list
    pub description: String
}
