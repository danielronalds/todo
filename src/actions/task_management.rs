use colored::Colorize;

use crate::task::Task;
use crate::task::TaskStatus;

use crate::user_config::UserConfig;


/// Updates the given task's status to InProgress
///
/// Parameters
/// task:  The task to change the status of
pub fn start_task(task: &mut Task)  -> Result<String, &'static str> {
    match task.status {
        // If the task is not started, start it 
        TaskStatus::NotStarted => {
            task.status = TaskStatus::InProgress;
            return Ok(format!("Started task '{}'", task.desc))
        },
        // Otherwise inform the user of the tasks' current status
        _ => return Err("Task already in progress!"),
    }
}


/// Updates the given task's status to Completed
///
/// Parameters
/// task:  The task to change the status of
pub fn finish_task(task: &mut Task) -> Result<String, &'static str> {
    match task.status {
        // Inform the user if the task has already been completed
        TaskStatus::Completed => return Err("Task already completed!"),
        // If the task is either not started or in progress, complete it
        _ => {
            task.status = TaskStatus::Completed;
            return Ok(format!("Completed '{}'!", task.desc))
        },
    }
}


/// Updates the given task's status to NotStarted
///
/// Parameters
/// task:  The task to change the status of
pub fn restart_task(task: &mut Task) -> String {
    task.status = TaskStatus::NotStarted;

    format!("Restarted task '{}'!", task.desc)
}


/// Creates a new Task and adds it the given vec
///
/// Parameters
/// tasks:  The vec to add the new task to
/// desc:   The description of the task
/// list:   The tasklist to add the task to
pub fn add_task(tasks: &mut Vec<Task>, desc: String, list: String) 
    -> Result<String, &'static str> {
    // Creates the task and checks if it was created succesfully, returning an error if not
    let new_task = match Task::build(list, desc, TaskStatus::NotStarted) {
        Ok(new_task) => new_task,
        Err(err) => return Err(err),
    };

    let success = format!("Added task '{}'!", new_task.desc.clone());

    tasks.push(new_task);

    Ok(success)
}


/// Removes the task at the given index from the given vec
///
/// Parameters
/// tasks:      The vec to add the new task to
/// task_index: The index of the task to remove
pub fn remove_task(tasks: &mut Vec<Task>, task_index: usize) -> String {
    // Creating the message to return to the run function to print out
    let task_desc = &tasks[task_index].desc;

    // Creating the return message
    let message = format!("Removed task '{}'!", task_desc);

    // Remove the task from the tasklist
    tasks.remove(task_index);

    message
}


/// Updates the description of the given task
///
/// Parameters
/// task:       The task that will have its description updated
/// new_desc:   The new description of the task
pub fn update_task(task: &Task, new_desc: String) -> Result<(Task, String), &'static str> {
    // Creates a new task that the function returns, so that error checking of what a proper task
    // desciption should be doesn't have to be repeated twice, meaning that if the requirments 
    // changed, this code wouldn't have to be
    let new_task = match Task::build(task.list.clone(), new_desc, task.status.clone()) {
        Ok(new_task) => new_task,
        Err(err) => return Err(err),
    };

    let success_message = format!("Updated Task to '{}'", &new_task.desc);

    Ok((new_task, success_message))
}


/// Sorts the given Task vec in descending order, with the completed tasks being at the top
///
/// Parameters
/// tasks:  The vec to sort
pub fn sort_tasks(tasks: Vec<Task>) -> Vec<Task> {
    // Declaring a vec to store sorted tasks, and an array of vecs for sorting
    let mut sorted_tasks: Vec<Task> = Vec::new();
    
    let mut sorting_vecs: [Vec<Task>; 3] = Default::default();

    // Sorting tasks
    for task in tasks {
        match task.status {
            TaskStatus::Completed => sorting_vecs[0].push(task),
            TaskStatus::InProgress => sorting_vecs[1].push(task),
            TaskStatus::NotStarted => sorting_vecs[2].push(task),
        } 
    }

    // Combining all the sorted vecs into one vec to return
    for tasks in sorting_vecs {
        for task in tasks {
            sorted_tasks.push(task);
        } 
    } 

    sorted_tasks
}


/// Deletes all completed tasks in the given Task vec
///
/// Parameters
/// tasks:  The vec to delete completed tasks from
pub fn cleanup_list(tasks: &mut Vec<Task>) -> String {
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

    let success_message = format!("Removed {} Completed tasks!", tasks_to_remove.len());

    for index in tasks_to_remove {
        tasks.remove(index);
    }

    success_message
}


/// Lists all tasks regardless of the current tasklist
///
/// Parameters
/// main_tasks:     The tasks in the current tasklist
/// other_tasks:    The rest of the tasks
/// config:         The user's config 
pub fn list_all_tasks(mut main_tasks: Vec<Task>, mut other_tasks: Vec<Task>, config: &UserConfig) {
    // Collecting all tasks into one vec
    let mut tasks: Vec<Task> = Vec::new();
    tasks.append(&mut main_tasks);
    tasks.append(&mut other_tasks);
    
    // Looping through every tasklist in the user's config
    for tasklist in &config.tasklists {
        // Printing a title for the tasks to be listed under
        println!("{}", tasklist.bold());

        for task in &tasks {
            // Loops through the task vec, printing tasks tagged with the current tasklist
            if &task.list == tasklist {
                println!("{}", task.to_string());
            }
        }

        // New line between lists!
        println!("");
    }
}


/// Lists all the tasks in the current tasklist
///
/// Parameters
/// tasks:          The tasks in the current tasklist
/// users_config:   The user's settings, used to retrieve the current tasklist's name
pub fn list_tasks(tasks: &[Task], users_config: &UserConfig) -> Result<(), &'static str> {
    // Prints an output informing the user that there are no tasks if the tasklist is empty
    if tasks.is_empty() {
        return Err("No tasks found, Add a task with the add command!");
    }

    if users_config.display_list_name {
        println!("{}", users_config.current_list.bold());
    }

    let mut task_id = 1;

    for task in tasks {
        if users_config.always_show_id ||
           users_config.smart_id && tasks.len() >= users_config.smart_id_num.into() {
            println!("{}: {}", &task_id.to_string().bold(), task.to_string());
        } else {
            println!("{}", task.to_string());
        }
        task_id += 1;
    }

    Ok(())
}
