use crate::task::{Task, TaskErrors, TaskStatus};

/// Enum for storing TaskManagementErrors
#[derive(Debug, PartialEq)]
pub enum TaskManagementErrors {
    TaskAlreadyGivenStatus,
    TaskDoesntExist,
    EmptyTasklist,
}

/// Lists all of the tasks in the tasks vec
///
/// Parameters
/// tasks:   The vec of tasks to list
pub fn list_tasks(tasks: &Vec<Task>) -> Result<(), TaskManagementErrors> {
    if tasks.is_empty() {
        return Err(TaskManagementErrors::EmptyTasklist);
    }

    for task in tasks {
        println!("{}", task.to_string());
    }

    Ok(())
}

/// Updates the task at the given index in the task vec to the given status
///
/// Parameters
/// tasks:        The vec of the task belongs to
/// index:        The index of the task to update
/// new_status:   The new status of the task
pub fn update_task_status(
    tasks: &mut Vec<Task>,
    index: usize,
    new_status: TaskStatus,
) -> Result<(), TaskManagementErrors> {
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

/// Deletes the task at the given index out of the given Vec<Task>
///
/// Parameters
/// tasks:   The vec to remove the task from
/// index:   The index of the task to remove (0 being the first task)
pub fn delete_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), TaskManagementErrors> {
    if index >= tasks.len() {
        return Err(TaskManagementErrors::TaskDoesntExist);
    }

    tasks.remove(index);

    Ok(())
}

/// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if the update_task_status function works
    fn update_task_status_works() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap(),
            Task::new(String::from("Another basic task!"), TaskStatus::InProgress).unwrap(),
        ];

        update_task_status(&mut tasks_vec, 1, TaskStatus::Completed).unwrap();

        assert_eq!(tasks_vec[1].status(), TaskStatus::Completed)
    }

    #[test]
    /// Tests if the update_task_status function returns the appropriate error if the task is the 
    /// given status already
    fn update_task_status_fails_when_already_at_given_status() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap(),
            Task::new(String::from("Another basic task!"), TaskStatus::InProgress).unwrap(),
        ];

        let err = update_task_status(&mut tasks_vec, 1, TaskStatus::InProgress).unwrap_err();

        assert_eq!(err, TaskManagementErrors::TaskAlreadyGivenStatus)
    }

    #[test]
    /// Tests if the update_task_status function returns the appropriate error if the given index 
    /// is out of range of the vec
    fn update_task_status_fails_when_index_out_of_range() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap(),
            Task::new(String::from("Another basic task!"), TaskStatus::NotStarted).unwrap(),
        ];

        let err = update_task_status(&mut tasks_vec, 3, TaskStatus::Completed).unwrap_err();

        assert_eq!(err, TaskManagementErrors::TaskDoesntExist)
    }

    #[test]
    /// Test if the update_task_status function returns the appropriate error if the vec is empty
    fn update_task_status_fails_on_empty_vec() {
        let mut tasks_vec: Vec<Task> = Vec::new();

        let err = update_task_status(&mut tasks_vec, 1, TaskStatus::Completed).unwrap_err();

        assert_eq!(err, TaskManagementErrors::EmptyTasklist)
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

    #[test]
    /// Tests if the delete_task function works
    fn delete_task_works() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap(),
            Task::new(String::from("Another basic task!"), TaskStatus::NotStarted).unwrap(),
        ];

        delete_task(&mut tasks_vec, 1).unwrap();

        assert_eq!(
            tasks_vec,
            vec![Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap()]
        )
    }

    #[test]
    /// Tests if the delete_task function errors when the index passed to it is out of range
    fn delete_task_errors_on_invalid_index() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(String::from("A basic task!"), TaskStatus::NotStarted).unwrap(),
            Task::new(String::from("Another basic task!"), TaskStatus::NotStarted).unwrap(),
        ];

        let error = delete_task(&mut tasks_vec, 2).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskDoesntExist)
    }

    #[test]
    /// Tests if the list function errors on an empty vec. This is the only test written for the
    /// list function as how it formats and will be constantly changed so no point really trying to
    /// write a test for that part of the function
    fn list_tasks_errors_on_empty_vec() {
        let tasks_vec: Vec<Task> = Vec::new();

        let error = list_tasks(&tasks_vec).unwrap_err();

        assert_eq!(error, TaskManagementErrors::EmptyTasklist)
    }
}
