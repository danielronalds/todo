use std::process::exit;

use crate::task::Task;
use crate::task::TaskStatus;

use crate::task_exists;


// Function to update a task's status to inprogress
pub fn start_task(t: &mut Task) {
    match t.status {
        // If the task is not started, start it 
        TaskStatus::NotStarted => {
            t.status = TaskStatus::InProgress;
            println!("Started task '{}'", t.desc);
        },
        // Otherwise inform the user of the tasks' current status
        TaskStatus::InProgress => println!("Task already in progress!"),
        TaskStatus::Completed => println!("Task already completed!"),
    }
}


// Function to update a task's status to completed
pub fn finish_task(t: &mut Task) {
    match t.status {
        // If the task is either not started or in progress, complete it
        TaskStatus::NotStarted => {
            t.status = TaskStatus::Completed;
            println!("Task '{}' completed!", t.desc);
        },
        TaskStatus::InProgress => {
            t.status = TaskStatus::Completed;
            println!("Task '{}' completed!", t.desc);
        },
        // Otherwise inform the user it has already been completed
        TaskStatus::Completed => println!("Task already completed!"),
    }
}


// Adds a task to a tasks vec
pub fn add_task(tasks: &mut Vec<Task>, desc: String) {
    let new_task = Task::build(&desc, TaskStatus::NotStarted).unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    println!("Task '{}' added!", &new_task.desc);

    tasks.push(new_task);
}


// Removes a task from a tasks vec
pub fn remove_task(tasks: &mut Vec<Task>, task_index: usize) {
    // Removing 1 off error
    let task_index = task_index - 1;

    // Checks to make sure the task is in range to prevent panic!
    task_exists(task_index, tasks);

    // Printing out the task description so the user knows what task was deleted
    let task_desc = &tasks[task_index].desc;
    println!("Task '{}' removed!", task_desc);

    // Remove the task from the tasklist
    tasks.remove(task_index);
}


// Function to list task
pub fn list_tasks(tasks: &[Task]) {
    let mut task_id = 1;

    for task in tasks {
        println!("{}", task.to_string(&task_id));
        task_id += 1;
    }
}


