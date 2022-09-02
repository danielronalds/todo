use colored::Colorize;

use crate::task::Task;
use crate::task::TaskStatus;

use crate::task_exists;
use crate::print_success;
use crate::print_error;
use crate::user_config::UserConfig;


// Function to update a task's status to inprogress
pub fn start_task(t: &mut Task) {
    match t.status {
        // If the task is not started, start it 
        TaskStatus::NotStarted => {
            t.status = TaskStatus::InProgress;
            print_success(format!("Started task '{}'", t.desc).as_str());
        },
        // Otherwise inform the user of the tasks' current status
        _ => print_error("Task already in progress!"),
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
            print_success(format!("Task '{}' completed!", t.desc).as_str());
        },
    }
}


// Function to set the status of any task to NotStarted
pub fn restart_task(t: &mut Task) {
    t.status = TaskStatus::NotStarted;
    print_success(format!("Task '{}' restarted!", t.desc).as_str());
}


// Adds a task to a tasks vec
pub fn add_task(tasks: &mut Vec<Task>, desc: String) -> Result<(), &'static str>{
    let new_task = Task::build(desc, TaskStatus::NotStarted);

    // Checks to see if the task was created succesfully, returning an error if not
    match &new_task {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    // Unwrapping the task if it was created succesfully
    let new_task = new_task.unwrap();

    print_success(format!("Task '{}' added!", &new_task.desc).as_str());

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
    print_success(format!("Task '{}' removed!", task_desc).as_str());

    // Remove the task from the tasklist
    tasks.remove(task_index);
}


// Function that updates the given tasks description
pub fn update_task(task: &Task, new_desc: String) -> Result<Task, &'static str> {
    // Creates a new task that the function returns, so that error checking of what a proper task
    // desciption should be doesn't have to be repeated twice, meaning that if the requirments 
    // changed, this code wouldn't have to be
    let new_task = Task::build(new_desc.to_string(), task.status.clone());

    // Checking if it's safe to unwrap the task and return it, to avoid a panic!
    match new_task {
        Ok(_) => (),
        Err(err) => return Err(err),
    }

    print_success(format!("Updated Task to '{}'", new_desc).as_str());

    Ok(new_task.unwrap())
}


// Function to sort tasks from completed to not started
pub fn sort_tasks(tasks: Vec<Task>) -> Vec<Task> {
    // Declaring a vec to store sorted tasks, and an array of vecs for sorting
    let mut sorted_tasks: Vec<Task> = Vec::new();
    
    let mut sorting_tasks: [Vec<Task>; 3] = Default::default();

    // Sorting tasks
    for task in tasks {
        match task.status {
            TaskStatus::Completed => sorting_tasks[0].push(task),
            TaskStatus::InProgress => sorting_tasks[1].push(task),
            TaskStatus::NotStarted => sorting_tasks[2].push(task),
        } 
    }

    // Combining all the sorted vecs into one vec to return
    for tasks in sorting_tasks {
        for task in tasks {
            sorted_tasks.push(task);
        } 
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

    print_success(format!("Removed {} Completed tasks!", tasks_to_remove.len()).as_str());

    for index in tasks_to_remove {
        tasks.remove(index);
    }
}


// Function to list task
pub fn list_tasks(tasks: &[Task], users_config: &UserConfig) {
    // Prints an output informing the user that there are no tasks if the tasklist is empty
    if tasks.is_empty() {
        print_error("No tasks found, Add a task with the add command!");
    }

    let mut task_id = 1;

    for task in tasks {
        if users_config.always_show_id ||
           users_config.smart_id && tasks.len() >= users_config.smart_id_num.into() {
            println!("{}: {}", &task_id.to_string().bold(), task.to_string(&task_id));
        } else {
            println!("{}", task.to_string(&task_id));
        }
        task_id += 1;
    }
}


