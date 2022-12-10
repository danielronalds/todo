use crate::task::{Task, TaskErrors, TaskStatus};

use crate::config::Config;

use colored::Colorize;

/// Enum for storing TaskManagementErrors
#[derive(Debug, PartialEq, Eq)]
pub enum TaskManagementErrors {
    TaskAlreadyGivenStatus,
    TaskDoesntExist,
    EmptyTasklist,
}

/// Lists all of the tasks in the tasks vec
///
/// Parameters
/// tasks:    The vec of tasks to list
/// config:   The user's config
pub fn list_tasks(tasks: &Vec<Task>, config: &Config) -> Result<(), TaskManagementErrors> {
    if tasks.is_empty() {
        return Err(TaskManagementErrors::EmptyTasklist);
    }

    // Checking if the listname should be printed
    if config.always_show_list_names() || (config.smart_list_names() && config.lists_len() > 1) {
        println!("{}", config.current_list().bold());
    }

    let mut task_id = 1;

    for task in tasks {
        if config.always_show_task_ids() {
            println!("{}. {}", task_id, task);
        } else {
            println!("{}", task);
        }

        task_id += 1;
    }

    Ok(())
}

/// Sorts the given task vec in the order Completed, InProgress, NotStarted
///
/// Parameters:
/// tasks:   The tasks vec to sort
pub fn sort_tasks(tasks: &mut Vec<Task>) -> Result<(), TaskManagementErrors> {
    // Returning an error if the given vec is empty
    if tasks.is_empty() {
        return Err(TaskManagementErrors::EmptyTasklist);
    }

    // Declaring an array of vecs for sorting
    let mut sorting_vecs: [Vec<Task>; 3] = Default::default();

    for task in tasks.iter_mut() {
        match task.status() {
            TaskStatus::Completed => sorting_vecs[0].push(task.clone()),
            TaskStatus::InProgress => sorting_vecs[1].push(task.clone()),
            TaskStatus::NotStarted => sorting_vecs[2].push(task.clone()),
        }
    }

    // Clearing the tasks vec
    tasks.clear();

    // Adding the tasks back into the given tasks vec in the sorted order
    tasks.extend(sorting_vecs[0].iter().cloned());
    tasks.extend(sorting_vecs[1].iter().cloned());
    tasks.extend(sorting_vecs[2].iter().cloned());

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
    if tasks.is_empty() {
        return Err(TaskManagementErrors::EmptyTasklist);
    }

    if index >= tasks.len() {
        return Err(TaskManagementErrors::TaskDoesntExist);
    }

    if tasks[index].status() == new_status {
        return Err(TaskManagementErrors::TaskAlreadyGivenStatus);
    }

    tasks[index].update_status(new_status);
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
/// Enum for representing errors with the update_task_description method. This will probably have
/// to be refactored/changed/scrapped later, however for now this should work?
pub enum UpdateTaskErrors {
    ManagementErrors(TaskManagementErrors),
    TaskErrors(TaskErrors),
}

/// Updates the desciption of the given task
///
/// Parameters
/// tasks:             The vec of the task belongs to
/// index:             The index of the task to update
/// new_description:   The new description of the task
pub fn update_task_description(
    tasks: &mut Vec<Task>,
    index: usize,
    new_description: String,
) -> Result<(), UpdateTaskErrors> {
    if tasks.is_empty() {
        return Err(UpdateTaskErrors::ManagementErrors(
            TaskManagementErrors::EmptyTasklist,
        ));
    }

    if index >= tasks.len() {
        return Err(UpdateTaskErrors::ManagementErrors(
            TaskManagementErrors::TaskDoesntExist,
        ));
    }

    match tasks[index].update_description(new_description) {
        Ok(_) => Ok(()),
        // Returning any errors from the updating of the task, wrapped in the UpdateTaskErrors enum
        Err(err) => Err(UpdateTaskErrors::TaskErrors(err)),
    }
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
    /// Tests if the update_task_description works
    fn update_task_description_works() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
        ];

        let new_description = String::from("New description");

        update_task_description(&mut tasks_vec, 1, new_description.clone()).unwrap();

        assert_eq!(new_description, tasks_vec[1].description());
    }

    #[test]
    /// Tests if the update_task_description returns the right error on an empty new_description
    fn update_task_description_fails_on_empty_new_description() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
        ];

        let new_description = String::new();

        let error = update_task_description(&mut tasks_vec, 1, new_description).unwrap_err();

        assert_eq!(
            error,
            UpdateTaskErrors::TaskErrors(TaskErrors::EmptyDescription)
        )
    }

    #[test]
    /// Tests if the update_task_description returns the right error on an invalid index
    fn update_task_description_fails_on_invalid_index() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
        ];

        let new_description = String::from("New description");

        let error = update_task_description(&mut tasks_vec, 2, new_description).unwrap_err();

        assert_eq!(
            error,
            UpdateTaskErrors::ManagementErrors(TaskManagementErrors::TaskDoesntExist)
        )
    }

    #[test]
    /// Tests if the update_task_description returns the right error on an empty vec
    fn update_task_description_fails_on_empty_vec() {
        let mut tasks_vec: Vec<Task> = Vec::new();

        let new_description = String::from("New description");

        let error = update_task_description(&mut tasks_vec, 1, new_description).unwrap_err();

        assert_eq!(
            error,
            UpdateTaskErrors::ManagementErrors(TaskManagementErrors::EmptyTasklist)
        )
    }

    #[test]
    /// Tests if the update_task_status function works
    fn update_task_status_works() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
        ];

        update_task_status(&mut tasks_vec, 1, TaskStatus::Completed).unwrap();

        assert_eq!(tasks_vec[1].status(), TaskStatus::Completed)
    }

    #[test]
    /// Tests if the update_task_status function returns the appropriate error if the task is the
    /// given status already
    fn update_task_status_fails_when_already_at_given_status() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
        ];

        let err = update_task_status(&mut tasks_vec, 1, TaskStatus::InProgress).unwrap_err();

        assert_eq!(err, TaskManagementErrors::TaskAlreadyGivenStatus)
    }

    #[test]
    /// Tests if the update_task_status function returns the appropriate error if the given index
    /// is out of range of the vec
    fn update_task_status_fails_when_index_out_of_range() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
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
    /// Tests if the delete_task function works
    fn delete_task_works() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
        ];

        delete_task(&mut tasks_vec, 1).unwrap();

        assert_eq!(
            tasks_vec,
            vec![Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main")
            )
            .unwrap()]
        )
    }

    #[test]
    /// Tests if the delete_task function errors when the index passed to it is out of range
    fn delete_task_errors_on_invalid_index() {
        let mut tasks_vec: Vec<Task> = vec![
            Task::new(
                String::from("A basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
        ];

        let error = delete_task(&mut tasks_vec, 2).unwrap_err();

        assert_eq!(error, TaskManagementErrors::TaskDoesntExist)
    }

    #[test]
    /// Tests if the list function errors on an empty vec. This is the only test written for the
    /// list function as how it formats and will be constantly changed so no point really trying to
    /// write a test for that part of the function
    fn list_tasks_errors_on_empty_vec() {
        let config = Config::new();

        let tasks_vec: Vec<Task> = Vec::new();

        let error = list_tasks(&tasks_vec, &config).unwrap_err();

        assert_eq!(error, TaskManagementErrors::EmptyTasklist)
    }

    #[test]
    /// Tests if the sort_tasks function works
    fn sort_tasks_works() {
        let mut tasks = vec![
            Task::new(
                String::from("A Completed task!"),
                TaskStatus::Completed,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("A NotStarted task!"),
                TaskStatus::NotStarted,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("An InProgress task!"),
                TaskStatus::InProgress,
                String::from("main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another Completed task!"),
                TaskStatus::Completed,
                String::from("main"),
            )
            .unwrap(),
        ];

        sort_tasks(&mut tasks).unwrap();

        assert_eq!(
            tasks,
            vec![
                Task::new(
                    String::from("A Completed task!"),
                    TaskStatus::Completed,
                    String::from("main")
                )
                .unwrap(),
                Task::new(
                    String::from("Another Completed task!"),
                    TaskStatus::Completed,
                    String::from("main")
                )
                .unwrap(),
                Task::new(
                    String::from("An InProgress task!"),
                    TaskStatus::InProgress,
                    String::from("main")
                )
                .unwrap(),
                Task::new(
                    String::from("A NotStarted task!"),
                    TaskStatus::NotStarted,
                    String::from("main")
                )
                .unwrap(),
            ]
        );
    }

    #[test]
    /// Tests if the sort_tasks function returns the correct error when it is passed an empty vec
    fn sort_tasks_fails_on_empty_vec() {
        let mut tasks = Vec::new();

        let error = sort_tasks(&mut tasks).unwrap_err();

        assert_eq!(error, TaskManagementErrors::EmptyTasklist)
    }
}
