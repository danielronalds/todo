#[derive(Debug)]
pub enum TaskStatus {
    Completed,
    InProgress,
    NotStarted,
}

pub struct Task {
    pub id: u32,
    pub desc: String,
    pub status: TaskStatus,
}
