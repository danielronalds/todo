// Public facing modules
pub mod args;
pub mod task;

// Private modules
mod task_management;

use crate::task::{Task, TaskStatus, TaskErrors};
use crate::args::AddCommand;

/// Creates a new task. This handles any errors and returns an appropriate error message
/// This approach will most likely change however, or this function moved
///
/// Parameters
/// arguments:   The arguments for the command from the cli
pub fn new_task(arguments: AddCommand) -> Result<Task, &'static str> {
    let task = match Task::new(arguments.description, TaskStatus::NotStarted) {
        Ok(task) => task,
        Err(err) => match err {
            TaskErrors::EmptyDescription => {
                return Err("Tasks cannot have empty descriptions!");
            }
            TaskErrors::InvalidCharInDescription => {
                return Err("Tasks cannot have the | char!");
            }
        },
    };

    Ok(task)
}
