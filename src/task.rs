pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

pub struct Task {
    pub desc: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn status_to_string(&self) -> String {
        match &self.status {
            TaskStatus::Completed => String::from("Completed"),
            TaskStatus::InProgress => String::from("InProgress"),
            TaskStatus::NotStarted => String::from("NotStarted"),
        }
    }
}
