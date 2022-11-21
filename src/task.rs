/// Enum for representing the status of a task
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

/// Enum for storing possible errors
pub enum Errors {}

/// Struct to represent a task
pub struct Task {
    description: String,
    status: TaskStatus
}

impl Task {}

#[cfg(test)]
mod tests {
    #[test]
    /// Checks if the constructor works with the expected input
    fn constructor_works() {
        unimplemented!();
    }
}
