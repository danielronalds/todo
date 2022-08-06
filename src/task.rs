#[derive(Debug)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

pub struct Task {
    pub desc: String,
    pub status: TaskStatus,
}
