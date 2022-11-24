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

#[cfg(test)]
mod test {
    use super::*;
    use crate::args::AddCommand;

    #[test]
    /// Tests if the new_task command works as expected
    fn new_task_works() {
        let description = String::from("This is a basic task");

        let expected_task = Task::new(description.clone(), TaskStatus::NotStarted).unwrap();

        let arguments = AddCommand{ description };

        let genereated_task = new_task(arguments).unwrap();

        assert_eq!(expected_task, genereated_task)
    }
}
