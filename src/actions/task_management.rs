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
        _ => println!("Task already in progress!"),
    }
}


// Function to update a task's status to completed
pub fn finish_task(t: &mut Task) {
    match t.status {
        // Inform the user if the task has already been completed
        TaskStatus::Completed => println!("Task already completed!"),
        // If the task is either not started or in progress, complete it
        _ => {
            t.status = TaskStatus::Completed;
            println!("Task '{}' completed!", t.desc);
        },
    }
}


// Function to set the status of any task to NotStarted
pub fn restart_task(t: &mut Task) {
    t.status = TaskStatus::NotStarted;
    println!("Task '{}' restarted!", t.desc);
}


// Adds a task to a tasks vec
pub fn add_task(tasks: &mut Vec<Task>, desc: String) -> Result<(), &'static str>{
    let new_task = Task::build(&desc, TaskStatus::NotStarted);

    // Checks to see if the task was created succesfully, returning an error if not
    match &new_task {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // Unwrapping the task if it was created succesfully
    let new_task = new_task.unwrap();

    println!("Task '{}' added!", &new_task.desc);

    tasks.push(new_task);

    Ok(())
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


// Function to sort tasks from completed to not started
pub fn sort_tasks(tasks: Vec<Task>) -> Vec<Task> {

    // This is probably a rough implementation, however it does work, and I don't think my
    // knowledge in rust is sufficient enough to simplify this code... yet

    // Declaring vecs to store sorted tasks
    let mut sorted_tasks: Vec<Task> = Vec::new();
    
    let mut completed_tasks: Vec<Task> = Vec::new();
    let mut inprogress_tasks: Vec<Task> = Vec::new();
    let mut notstarted_tasks: Vec<Task> = Vec::new();

    // Sorting tasks
    for task in tasks {
        match task.status {
            TaskStatus::Completed => completed_tasks.push(task),
            TaskStatus::InProgress => inprogress_tasks.push(task),
            TaskStatus::NotStarted => notstarted_tasks.push(task),
        } 
    }

    // Combining all the sorted vecs into one vec to return
    for task in completed_tasks {
        sorted_tasks.push(task);
    }

    for task in inprogress_tasks {
        sorted_tasks.push(task);
    }

    for task in notstarted_tasks {
        sorted_tasks.push(task);
    }

    sorted_tasks
}


// Function to delete completed tasks from the task list
pub fn cleanup_list(tasks: &mut Vec<Task>) {
    let mut tasks_to_remove: Vec<usize> = Vec::new();

    // Collects the index's of completed tasks in reverse order so that when deleting tasks, the 
    // index of the next task to get deleted changes due to an element before it being removed 
    // from the list
    let mut current_index = tasks.len();

    while current_index > 0 {
        // Removes 1 off the current_index first, so that it doesn't panic when attempting to
        // access an out of bounds element of the vec
        current_index -= 1;

        match tasks[current_index].status {
            TaskStatus::Completed => tasks_to_remove.push(current_index),
            _ => continue,
        }
    }

    println!("Removed {} Completed tasks!", tasks_to_remove.len());

    for index in tasks_to_remove {
        tasks.remove(index);
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


