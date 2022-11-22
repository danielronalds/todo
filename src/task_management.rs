use crate::task::{Task, TaskStatus, TaskErrors};

/// Enum for storing TaskManagementErrors
#[derive(Debug, PartialEq)]
pub enum TaskManagementErrors {
    TaskAlreadyGivenStatus,
}

/// Sets the status of the given task to InProgress
pub fn start_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    task.update_status(TaskStatus::InProgress);
    Ok(())
}

/// Sets the status of the given task to Completed
pub fn finish_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    task.update_status(TaskStatus::Completed);
    Ok(())
}

/// Sets the status of the given task to NotStarted
pub fn restart_task(task: &mut Task) -> Result<(), TaskManagementErrors> {
    task.update_status(TaskStatus::NotStarted);
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

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        let error = start_task(&mut task).unwrap_err();

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

        let error = start_task(&mut task).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskAlreadyGivenStatus)
    }
}
