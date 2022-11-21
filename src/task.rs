/// Enum for representing the status of a task
#[derive(Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

/// Enum for storing possible errors
#[derive(Debug)]
pub enum Errors {}

/// Struct to represent a task
pub struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    /// Returns a new Task struct with the desciption and status passed in
    ///
    /// Parameters
    /// description:   The task's desciption
    /// status:        The task's status
    pub fn new(description: String, status: TaskStatus) -> Result<Task, Errors> {
        Ok(Task {
            description: String::new(),
            status: TaskStatus::NotStarted,
        })
    }

    /// Returns a clone of the tasks desciption
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Returns the tasks status as a clone
    pub fn status(&self) -> TaskStatus {
        self.status.clone()
    }
}

mod tests {
    use super::*;

    #[test]
    /// Checks if the constructor works with the expected input
    fn constructor_works() {
        let description = String::from("This is a simple task!");

        let task = Task::new(description.clone(), TaskStatus::NotStarted).unwrap();

        assert_eq!(task.description(), description);
    }

    #[test]
    /// Checks if the constructor creates the task with the right task status
    fn constructor_right_status() {
        let description = String::from("This is a simple task!");

        let task = Task::new(description.clone(), TaskStatus::Completed).unwrap();

        assert_eq!(task.status(), TaskStatus::Completed)
    }
}
