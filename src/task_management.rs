use crate::task::{Task, TaskStatus, TaskErrors};

/// Sets the status of the given task to InProgress
pub fn start_task(task: &mut Task) {
    task.update_status(TaskStatus::InProgress);
}

/// Sets the status of the given task to Completed
pub fn finish_task(task: &mut Task) {
    task.update_status(TaskStatus::Completed);
}

/// Sets the status of the given task to NotStarted
pub fn restart_task(task: &mut Task) {
    task.update_status(TaskStatus::NotStarted);
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

        start_task(&mut task);

        assert_eq!(task.status(), TaskStatus::InProgress)
    }

    #[test]
    /// Tests if the finish_task function works
    fn finish_task_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::NotStarted).unwrap();

        finish_task(&mut task);

        assert_eq!(task.status(), TaskStatus::Completed)
    }

    #[test]
    /// Tests if the restart_task function works
    fn restart_task_works() {
        let description = String::from("This is a basic task");

        let mut task = Task::new(description, TaskStatus::InProgress).unwrap();

        restart_task(&mut task);

        assert_eq!(task.status(), TaskStatus::NotStarted)
    }
}
