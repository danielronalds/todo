use std::fmt;

use serde::{Deserialize, Serialize};

// Pretty output
use colored::Colorize;

/// Enum for representing the status of a task
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

/// Enum for storing possible errors
#[derive(Debug, PartialEq, Eq)]
pub enum TaskErrors {
    EmptyDescription,
    EmptyList
}

/// Struct to represent a task
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Task {
    description: String,
    status: TaskStatus,
    list: String
}

impl Task {
    /// Returns a new Task struct with the description and status passed in
    ///
    /// Parameters
    /// description:   The task's description
    /// status:        The task's status
    /// list:          The list the task belongs to
    pub fn new(description: String, status: TaskStatus, list: String) -> Result<Task, TaskErrors> {
        // Return an error if the description is empty
        if description.is_empty() {
            return Err(TaskErrors::EmptyDescription);
        }

        if list.is_empty() {
            return Err(TaskErrors::EmptyList);
        }

        Ok(Task {
            description,
            status,
            list
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

    /// Returns the tasks list as a clone
    pub fn list(&self) -> String {
        self.list.clone()
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

    /// TODO: REMOVE THIS 
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
    #![allow(unused_imports)]
    // For some reason, clippy says this isn't needed, howeveer deleting it breaks everything so
    // I've attached the allow unused_imports atribute
    use super::*;

    #[test]
    /// Checks if the constructor works with the expected input
    fn constructor_right_description() {
        let description = String::from("This is a simple task!");

        let list = String::from("main");

        let task = Task::new(description.clone(), TaskStatus::NotStarted, list).unwrap();

        assert_eq!(task.description(), description);
    }

    #[test]
    /// Checks if the constructor creates the task with the right task status
    fn constructor_right_status() {
        let description = String::from("This is a simple task!");

        let list = String::from("main");

        let task = Task::new(description, TaskStatus::Completed, list).unwrap();

        assert_eq!(task.status(), TaskStatus::Completed)
    }

    #[test]
    /// Checks if the constructor uses the right list
    fn constructor_right_list() {
        let description = String::from("This is a simple task!");

        let list = String::from("main");

        let task = Task::new(description, TaskStatus::NotStarted, list.clone()).unwrap();

        assert_eq!(task.list(), list);
    }

    #[test]
    /// Checks if the constructor will provide the correct error when passed an empty description
    fn constructor_fails_on_empty_description() {
        let description = String::new();

        let list = String::from("main");

        let task_error = Task::new(description, TaskStatus::InProgress, list).unwrap_err();

        assert_eq!(task_error, TaskErrors::EmptyDescription)
    }

    #[test]
    /// Checks if the constructor will provide the right error when handed an empty list
    fn constructor_fails_on_empty_list() {
        let description = String::from("This is a simple task!");

        let list = String::new();

        let error = Task::new(description, TaskStatus::NotStarted, list).unwrap_err();

        assert_eq!(error, TaskErrors::EmptyList);
    }

    #[test]
    /// Checks if the update_status method works
    fn update_status_works() {
        let description = String::from("This is a basic task!");

        let list = String::from("main");

        let mut task = Task::new(description, TaskStatus::NotStarted, list).unwrap();

        task.update_status(TaskStatus::InProgress);

        assert_eq!(task.status(), TaskStatus::InProgress)
    }

    #[test]
    /// Checks if the update_description method works
    fn update_description_works() {
        let description = String::from("This is the first description");

        let list = String::from("main");

        let mut task = Task::new(description, TaskStatus::InProgress, list).unwrap();

        let new_description = String::from("The is the new description");

        task.update_description(new_description.clone()).unwrap();

        assert_eq!(task.description(), new_description)
    }

    #[test]
    /// Checks if the update_description fails when passed an empty description
    fn update_description_fails_on_empty_description() {
        let description = String::from("This is the first description");

        let list = String::from("main");

        let mut task = Task::new(description, TaskStatus::InProgress, list).unwrap();

        let new_description = String::new();

        let err = task
            .update_description(new_description.clone())
            .unwrap_err();

        assert_eq!(err, TaskErrors::EmptyDescription)
    }

    #[test]
    /// Checks if the to_save_string method works
    fn to_save_string_works() {
        let description = String::from("This is a saved task!");

        let list = String::from("main");

        let task = Task::new(description, TaskStatus::NotStarted, list).unwrap();
        
        let expected_output = String::from("This is a saved task!|NotStarted");

        assert_eq!(expected_output, task.to_save_string())
    }
}
