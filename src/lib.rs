// Public facing modules
pub mod args;
pub mod task;

// Private modules
mod task_management;

use crate::args::{AddCommand, DeleteCommand, StartCommand, FinishCommand, RestartCommand};
use crate::task::{Task, TaskErrors, TaskStatus};
use crate::task_management::TaskManagementErrors;

/// Lists the tasks in the given vec
///
/// Parameters
/// tasks:   The task vec to list
pub fn list_tasks(tasks: &Vec<Task>) -> Result<(), &'static str> {
    match task_management::list_tasks(tasks) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            TaskManagementErrors::EmptyTasklist => return Err("There are no tasks in the list!"),
            // Covering any other errors for now in case the function changes
            _ => return Err("An unknown error has occured!"),
        },
    }
}

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

/// Deletes a task from the list. This function handles the errors and returns a str containing a
/// messge to print
///
/// Parameters
/// tasks:       The task vec to delete from
/// arguments:   The arguments for the command from the cli
pub fn delete_task(tasks: &mut Vec<Task>, arguments: DeleteCommand) -> &'static str {
    // Taking one off of the index as Task ID's start at 1 not 0
    let index = arguments.task_id - 1;

    match task_management::delete_task(tasks, index) {
        Ok(_) => "Task deleted!",
        // There is only one possible error for this so no need to match them
        Err(_) => "Task doesn't exist!",
    }
}

/// Starts the task at the given id in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn start_task(tasks: &mut Vec<Task>, arguments: StartCommand) -> &'static str {
    // Taking one off of the index as Task ID's start at 1 not 0
    let index = arguments.task_id - 1;

    match task_management::update_task_status(tasks, index, TaskStatus::InProgress) {
        Ok(_) => "Task has been started!",
        Err(err) => match err {
            TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already In Progress",
            TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
            TaskManagementErrors::EmptyTasklist => "No tasks found!",
        },
    }
}

/// Finishes the task at the given id in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn finish_task(tasks: &mut Vec<Task>, arguments: FinishCommand) -> &'static str {
    // Taking one off of the index as Task ID's start at 1 not 0
    let index = arguments.task_id - 1;

    match task_management::update_task_status(tasks, index, TaskStatus::Completed) {
        Ok(_) => "Task has been completed!",
        Err(err) => match err {
            TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already Completed",
            TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
            TaskManagementErrors::EmptyTasklist => "No tasks found!",
        },
    }
}

/// Restarts the task at the given id in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn restart_task(tasks: &mut Vec<Task>, arguments: RestartCommand) -> &'static str {
    // Taking one off of the index as Task ID's start at 1 not 0
    let index = arguments.task_id - 1;

    match task_management::update_task_status(tasks, index, TaskStatus::NotStarted) {
        Ok(_) => "Task has been restarted!",
        Err(err) => match err {
            TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already Not Started",
            TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
            TaskManagementErrors::EmptyTasklist => "No tasks found!",
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::args::AddCommand;

    #[test]
    /// Tests if the new_task command works as expected
    fn new_task_works() {
        let description = String::from("This is a basic task");

        let expected_task = Task::new(description.clone(), TaskStatus::NotStarted).unwrap();

        let arguments = AddCommand { description };

        let genereated_task = new_task(arguments).unwrap();

        assert_eq!(expected_task, genereated_task)
    }
}
