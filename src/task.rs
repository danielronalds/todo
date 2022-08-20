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
    // Constructor
    pub fn new(task_desc: &str, task_status: TaskStatus) -> Task {
        Task {
            desc: task_desc.to_string(),
            status: task_status,
        }
    }

    // Method to output the task's status to a String
    pub fn status_to_string(&self) -> String {
        match &self.status {
            TaskStatus::Completed => String::from("Completed"),
            TaskStatus::InProgress => String::from("InProgress"),
            TaskStatus::NotStarted => String::from("NotStarted"),
        }
    }

    // Method to return Task as String 
    pub fn to_string(&self, task_id: &u32) -> String {
        match &self.status {
            TaskStatus::NotStarted => String::from(format!("{}: [x] {}", task_id, &self.desc)),
            TaskStatus::InProgress => String::from(format!("{}: [~] {}", task_id, &self.desc)),
            TaskStatus::Completed => String::from(format!("{}: [✓] {}", task_id, &self.desc)),
        }
    }
}
