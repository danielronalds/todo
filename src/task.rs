use std::fmt;

// Pretty output
use colored::Colorize;

/// Enum for representing the status of a task
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

/// Enum for storing possible errors
#[derive(Debug, PartialEq, Eq)]
pub enum TaskErrors {
    EmptyDescription,
    InvalidCharInDescription,
}

/// Struct to represent a task
#[derive(Debug, PartialEq, Eq)]
pub struct Task {
    description: String,
    status: TaskStatus,
}

impl Task {
    /// Returns a new Task struct with the description and status passed in
    ///
    /// Parameters
    /// description:   The task's description
    /// status:        The task's status
    pub fn new(description: String, status: TaskStatus) -> Result<Task, TaskErrors> {
        // Return an error if the description is empty
        if description.is_empty() {
            return Err(TaskErrors::EmptyDescription);
        }

        // Return an error if the description contains a | char
        if description.contains('|') {
            return Err(TaskErrors::InvalidCharInDescription);
        }

        Ok(Task {
            description,
            status,
        })
    }

    /// Returns a clone of the tasks description
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// Returns the tasks status as a clone
    pub fn status(&self) -> TaskStatus {
        self.status.clone()
    }

    /// Updates the description of the task
    ///
    /// Parameters
    /// new_description:   The new description of the task
    pub fn update_description(&mut self, new_description: String) -> Result<(), TaskErrors> {
        // Return an error if the new description is empty
        if new_description.is_empty() {
            return Err(TaskErrors::EmptyDescription);
        }

        // Return an error if the new description contains a | char
        if new_description.contains('|') {
            return Err(TaskErrors::InvalidCharInDescription);
        }

        self.description = new_description;

        Ok(())
    }

    /// Updates the status of the task
    ///
    /// Parameters
    /// new_status:   The new status of the task
    pub fn update_status(&mut self, new_status: TaskStatus) {
        self.status = new_status;
    }

    /// Returns the struct as a string that can be written to a file
    pub fn to_save_string(&self) -> String {
        format!("{}|{:?}", self.description(), self.status())
    }
}

impl fmt::Display for Task {
    /// Returns the task as a 'pretty string'
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = format!("[{}]", match self.status() {
            TaskStatus::NotStarted => "x".bright_red(),
            TaskStatus::InProgress => "~".bright_yellow(),
            TaskStatus::Completed => "✔".bright_green(),
        }).bold();

        write!(f, "{} {}", status, self.description())

    }
}

/// Unit tests
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

        let task = Task::new(description, TaskStatus::Completed).unwrap();

        assert_eq!(task.status(), TaskStatus::Completed)
    }

    #[test]
    /// Checks if the constructor will provide the correct error when passed an empty description
    fn constructor_fails_on_empty_description() {
        let description = String::new();

        let task_error = Task::new(description, TaskStatus::InProgress).unwrap_err();

        assert_eq!(task_error, TaskErrors::EmptyDescription)
    }

    #[test]
    /// Checks if the constructor returns the right error when the description contains a '|' char
    fn constructor_fails_on_invalid_char() {
        let description = String::from("This invalid char | cannot be in the description");

        let task_error = Task::new(description, TaskStatus::InProgress).unwrap_err();

        assert_eq!(task_error, TaskErrors::InvalidCharInDescription)
    }

    #[test]
    /// Checks if the update_status method works
    fn update_status_works() {
        let description = String::from("This is a basic task!");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        task.update_status(TaskStatus::InProgress);

        assert_eq!(task.status(), TaskStatus::InProgress)
    }

    #[test]
    /// Checks if the update_description method works
    fn update_description_works() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let new_description = String::from("The is the new description");

        task.update_description(new_description.clone()).unwrap();

        assert_eq!(task.description(), new_description)
    }

    #[test]
    /// Checks if the update_description fails when passed an empty description
    fn update_description_fails_on_empty_description() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let new_description = String::new();

        let err = task
            .update_description(new_description.clone())
            .unwrap_err();

        assert_eq!(err, TaskErrors::EmptyDescription)
    }

    #[test]
    /// Checks if the update_description fails when passed a description with an invalid char
    fn update_description_fails_on_invalid_char() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let new_description = String::from("This invalid char | cannot be in the description");

        let err = task
            .update_description(new_description.clone())
            .unwrap_err();

        assert_eq!(err, TaskErrors::InvalidCharInDescription)
    }

    #[test]
    /// Checks if the to_save_string method works
    fn to_save_string_works() {
        let description = String::from("This is a saved task!");

        let task = Task::new(description, TaskStatus::NotStarted).unwrap();
        
        let expected_output = String::from("This is a saved task!|NotStarted");

        assert_eq!(expected_output, task.to_save_string())
    }
}
