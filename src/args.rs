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
    /// Sorts the tasks in the current tasklist 
    Sort,
    /// Add a task to the tasklist
    Add(AddCommand),
    /// Update an existing tasks description
    Update(UpdateCommand),
    /// Deletes a task from the tasklist
    Delete(DeleteCommand),
    /// Sets the status of a task to In Progress
    Start(StartCommand),
    /// Sets the status of a task to Completed
    Finish(FinishCommand),
    /// Sets the status of a task to Not Started
    Restart(RestartCommand),
    /// For viewing, creating, and deleting lists
    List(ListCommand),
    /// For configuring todo in this project
    Configure(ConfigCommand)
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

#[derive(Debug, Args)]
pub struct StartCommand {
    /// The Task ID of the task to start
    pub task_id: usize
}

#[derive(Debug, Args)]
pub struct FinishCommand {
    /// The Task ID of the task to finish
    pub task_id: usize
}

#[derive(Debug, Args)]
pub struct RestartCommand {
    /// The Task ID of the task to finish
    pub task_id: usize
}

#[derive(Debug, Args)]
pub struct UpdateCommand {
    /// The Task ID of the task to update
    pub task_id: usize,
    /// The new desciption of the task
    pub new_description: String
}

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(short, long)]
    /// Create a new list
    pub create: Option<String>,

    #[arg(short, long)]
    /// Switch to a list
    pub switch: Option<String>,

    #[arg(short, long)]
    /// Delete a list
    pub delete: Option<String>
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[arg(long)]
    pub always_show_task_ids: Option<bool>
}
