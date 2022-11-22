use crate::task::{Task, TaskStatus, TaskErrors};

/// Enum for storing TaskManagementErrors
#[derive(Debug, PartialEq)]
pub enum TaskManagementErrors {
    TaskAlreadyGivenStatus,
}

/// Sets the status of the given task to InProgress
///
/// Parameters
/// task:   The task to update the status of
pub fn start_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    if task.status() == TaskStatus::InProgress {
        return Err(TaskManagementErrors::TaskAlreadyGivenStatus);
    }

    task.update_status(TaskStatus::InProgress);
    Ok(())
}

/// Sets the status of the given task to Completed
///
/// Parameters
/// task:   The task to update the status of
pub fn finish_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    if task.status() == TaskStatus::Completed {
        return Err(TaskManagementErrors::TaskAlreadyGivenStatus);
    }

    task.update_status(TaskStatus::Completed);
    Ok(())
}

/// Sets the status of the given task to NotStarted
///
/// Parameters
/// task:   The task to update the status of
pub fn restart_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    if task.status() == TaskStatus::NotStarted {
        return Err(TaskManagementErrors::TaskAlreadyGivenStatus);
    }

    task.update_status(TaskStatus::NotStarted);
    Ok(())
}

/// Changes the desciption of the given task
///
/// Parameters
/// task:          The task to update the status of
/// description:   The new description of the task
pub fn update_task_description(task: &mut Task, description: String) -> Result<(), TaskErrors> {
    task.update_description(description)?;
    Ok(())
}

/// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if the start_task function works
    fn start_task_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        start_task(&mut task).unwrap();

        assert_eq!(task.status(), TaskStatus::InProgress)
    }

    #[test]
    /// Tests if the start_task function returns the appropriate error if the task is already
    /// started
    fn start_task_fails_when_already_started() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let error = start_task(&mut task).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskAlreadyGivenStatus)
    }

    #[test]
    /// Tests if the finish_task function works
    fn finish_task_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        finish_task(&mut task).unwrap();

        assert_eq!(task.status(), TaskStatus::Completed)
    }

    #[test]
    /// Tests if the finish_task function returns the appropriate error if the task is already
    /// finished
    fn finish_task_fails_when_already_finished() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::Completed).unwrap();

        let error = finish_task(&mut task).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskAlreadyGivenStatus)
    }

    #[test]
    /// Tests if the restart_task function works
    fn restart_task_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        restart_task(&mut task).unwrap();

        assert_eq!(task.status(), TaskStatus::NotStarted)
    }

    #[test]
    /// Tests if the restart_task function returns the appropriate error if the task is already
    /// not started
    fn restart_task_fails_when_already_not_started() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        let error = restart_task(&mut task).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskAlreadyGivenStatus)
    }

    #[test]
    /// Tests if the update_task_description function works
    fn update_task_description_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        let new_description = String::from("This is a new description");

        update_task_description(&mut task, new_description.clone()).unwrap();

        assert_eq!(task.description(), new_description)
    }

    #[test]
    /// Checks if the update_task_description function fails when passed an empty description
    fn update_description_fails_on_empty_description() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let new_description = String::new();

        let err = update_task_description(&mut task, new_description).unwrap_err();

        assert_eq!(err, TaskErrors::EmptyDescription)
    }

    #[test]
    /// Checks if the update_task_description function fails when passed a description with an 
    /// invalid char
    fn update_task_description_fails_on_invalid_char() {
        let description = String::from("This is the first description");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let new_description = String::from("This invalid char | cannot be in the description");

        let err = update_task_description(&mut task, new_description).unwrap_err();

        assert_eq!(err, TaskErrors::InvalidCharInDescription)
    }
}
