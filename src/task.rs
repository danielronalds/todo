// For pretty formating
use colored::Colorize;

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
    // Constructor for task struct
    pub fn build(task_desc: &str, task_status: TaskStatus) -> Result<Task, &'static str> {
        // Checking to see if the passed description is empty, returning an error result if it is
        if task_desc.is_empty() {
            return Err("Tasks cannot have an empty desciption!")
        }

        // Checks that the string doesn't contain a | character, as that would mess with saving the
        // tasks to the .tasks file as it is used as the seperator of the task desc, and status
        if task_desc.contains("|") {
            return Err("Tasks cannot contain the | character in their descriptions!")
        }

        // Otherwise returning a task struct
        Ok(Task {
            desc: task_desc.to_string(),
            status: task_status,
        })
    }


    // Method to output the task's status to a String
    pub fn status_to_string(&self) -> String {
        match &self.status {
            TaskStatus::NotStarted => String::from("NotStarted"),
            TaskStatus::InProgress => String::from("InProgress"),
            TaskStatus::Completed  => String::from("Completed"),
        }
    }


    // Method to return Task as String 
    pub fn to_string(&self, _task_id: &u32) -> String {
        let task_status = format!("[{}]", match &self.status {
            TaskStatus::NotStarted => format!("{}", "x".red()),
            TaskStatus::InProgress => format!("{}", "~".yellow()),
            TaskStatus::Completed  => format!("{}", "✔".green()),
        }).bold();

        // format!("{task_id}: {} {}", symbol, &self.desc)
        format!("{} {}", task_status, &self.desc)
    }
}
