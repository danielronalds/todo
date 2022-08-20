use crate::task::Task;
use crate::task::TaskStatus;


/// Function to update a task's status to inprogress
pub fn start_task(t: &mut Task) {
    match t.status {
        // If the task is not started, start it 
        TaskStatus::NotStarted => {
            t.status = TaskStatus::InProgress;
            println!("Started task '{}'\n", t.desc);
        },
        // Otherwise inform the user of the tasks' current status
        TaskStatus::InProgress => println!("Task already in progress!\n"),
        TaskStatus::Completed => println!("Task already completed!\n"),
    }
}


// Function to update a task's status to completed
pub fn finish_task(t: &mut Task) {
    match t.status {
        // If the task is either not started or in progress, complete it
        TaskStatus::NotStarted => {
            t.status = TaskStatus::Completed;
            println!("Task '{}' completed!\n", t.desc);
        },
        TaskStatus::InProgress => {
            t.status = TaskStatus::Completed;
            println!("Task '{}' completed!\n", t.desc);
        },
        // Otherwise inform the user it has already been completed
        TaskStatus::Completed => println!("Task already completed!\n"),
    }
}


// Adds a task to a tasks vec
pub fn add_task(tasks: &mut Vec<Task>, desc: String) {
    // Checks to see if the task is valid (has description)
    if !desc.is_empty() {
        // If it is, create a task and add it to the borrowed list 
        let new_task = Task {
            desc: String::from(desc),
            status: TaskStatus::NotStarted,
        };

        println!("Task '{}' added!\n", &new_task.desc);

        tasks.push(new_task);
    } else {
        // Informs the user if the task is not valid
        println!("Task has no description!\n")
    }
}


// Removes a task from a tasks vec
pub fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    let index = index - 1;
    // Checks to make sure the task is in range to prevent panic!
    if tasks.len() > index {
        // Adding 1 to index so the task id's start at 1
        let task_desc = &tasks[index].desc;
        println!("Task '{}' removed!\n", task_desc);

        // If the task does exist, remove it 
        tasks.remove(index);
    }
    else {
        // Informs the user if the task is not in range
        println!("Task does not exist!\n");
    }
}


// Function to list task
pub fn list_tasks(tasks: &[Task]) {
    let mut task_id = 1;

    for task in tasks {
        println!("{}", task.to_string(&task_id));
        task_id += 1;
    }
}


