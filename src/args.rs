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
    /// Lists the tasks in the current tasklist
    Tasks,
    /// Add a task to the tasklist
    Add(AddCommand),
    /// Deletes a task from the tasklist
    Delete(DeleteCommand)
}

#[derive(Debug, Args)]
pub struct AddCommand {
    /// The description of the task you're adding to the list
    pub description: String
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    /// The Task ID of the task to delete
    pub task_id: usize
}