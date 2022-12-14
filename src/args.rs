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
    /// Lists the tasks in the current list
    Tasks(TasksCommand),
    /// Sorts the tasks in the current list 
    Sort,
    /// Removes any completed tasks in the current list
    Cleanup(CleanupCommand),
    /// Add a task to the tasklist
    Add(AddCommand),
    /// Update an existing tasks description
    Update(UpdateCommand),
    /// Deletes a task from the list
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
    Config(ConfigCommand),
    /// Deletes the .todo directory, with the config and tasks file
    Nuke
}

#[derive(Debug, Args)]
pub struct TasksCommand {
    #[arg(short, long)]
    /// Lists all tasks instead of only the tasks in your current list
    pub all: bool,
}

#[derive(Debug, Args)]
pub struct CleanupCommand {
    #[arg(short, long)]
    pub all: bool,
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
    pub new: Option<String>,

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
    /// Whether task ids should always be shown in the tasks command
    pub always_show_task_ids: Option<bool>,

    #[arg(long)]
    /// Task IDs are only shown when there are more than the set amount of tasks
    pub smart_task_ids: Option<bool>,

    #[arg(long)]
    /// The number of tasks for task ids to be shown when smart_task_id is on
    pub num_of_tasks: Option<usize>,

    #[arg(long)]
    /// Whether the list name should be shown in the tasks command, override smart_list_names
    pub always_show_list_name: Option<bool>,

    #[arg(long)]
    /// Only shows list names when there is more than one list
    pub smart_list_names: Option<bool>
}
