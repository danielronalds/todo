use colored::Colorize;

use crate::task::Task;
use crate::task::TaskStatus;

use crate::print_success;
use crate::print_error;
use crate::user_config::UserConfig;


// All functions relating to managing tasklists


// Function to sort a tasklist and return a vec containing the tasks tagged with the current list
// as well as vec containing the rest
pub fn sort_by_current_list(current_list: String, tasks: Vec<Task>) -> (Vec<Task>, Vec<Task>) {
    let mut tagged_tasks: Vec<Task> = Vec::new();

    let mut other_tasks: Vec<Task> = Vec::new();

    for task in tasks {
        if task.list == current_list {
            tagged_tasks.push(task);
            continue;
        }
        other_tasks.push(task);
    }

    (tagged_tasks, other_tasks)
}


// Function to add a tasklist
pub fn add_tasklist(users_config: &mut UserConfig, new_tasklist: String) 
    -> Result<String, &'static str> {
    if new_tasklist.is_empty() {
        return Err("No tasklist name supplied!");
    }
    
    if new_tasklist.contains("|") {
        return Err("Tasklist names cannot contain the | character");
    }

    // Checks to see if the tasklist exists, reporting an error if it does
    for list_name in &users_config.tasklists {
        if &new_tasklist == list_name {
            return Err("Tasklist already exists!");
        }
    }

    let message = format!("Tasklist {} Added!", &new_tasklist);

    users_config.tasklists.push(new_tasklist);

    Ok(message)
}


// Function to update the current tasklists name
pub fn update_tasklist_name(users_config: &mut UserConfig, tasks: &mut Vec<Task>, new_name: String) 
    -> Result<&'static str, &'static str> {
    // Checking if the new name is empty or contains illegal characters
    if new_name.is_empty() {
        return Err("No tasklist name supplied!");
    }
    
    if new_name.contains("|") {
        return Err("Tasklist names cannot contain the | character");
    }

    // Removing old tasklist name from the list of tasklists
    users_config.tasklists.retain(|listname| {
        !(listname == &users_config.current_list)
    });

    // Adding new name to tasklists and setting it as the current list
    users_config.tasklists.push(new_name.clone());
    
    users_config.current_list = new_name.clone();

    // Updating all the tasks with the new name
    for task in tasks {
        task.list = new_name.clone();
    }

    Ok("Tasklist name updated!")
}


// Function to delete the current tasklist
pub fn delete_tasklist(users_config: &mut UserConfig, tasklist: &mut Vec<Task>) 
    -> Result<&'static str, &'static str> {
    // Checks to make sure that it is not the last checklist
    if users_config.tasklists.len() < 2 {
        return Err("Cannot delete this tasklist, you must have at least one tasklist!");
    }
    
    // Confirming the action with the user
    print_error("This will delete the current tasklist and all its tasks, are you sure? [y/N]\n");

    let mut confirmation = String::new();

    std::io::stdin().read_line(&mut confirmation).expect("Couldn't read line");

    match confirmation.trim() {
        "y" | "Y" | "yes" | "YES"  => {
            // Clearing all the tasks under the current tasklist
            tasklist.clear();

            // Removing tasklist from the list of tasklists
            users_config.tasklists.retain(|listname| {
                !(listname == &users_config.current_list)
            });

            // Updating Current tasklist to the first tasklist
            users_config.current_list = users_config.tasklists[0].clone();

            return Ok("Removed the tasklist succesfully!");
        },
        _ => return Err("Tasklist not deleted"),
    }

}


// All functions relating to managing tasks


// Function that lists all of the current tasklist
pub fn list_tasklists(user_config: &UserConfig) {
    for name in &user_config.tasklists {
        if name == &user_config.current_list {
            println!("{name} {}", "✔".green());
            continue
        }
        println!("{name}");
    }
}


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
        TaskStatus::Completed => print_error("Task already completed!"),
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
pub fn add_task(tasks: &mut Vec<Task>, desc: String, list: String) 
    -> Result<String, &'static str> {
    // Creates the task and checks if it was created succesfully, returning an error if not
    let new_task = match Task::build(list, desc, TaskStatus::NotStarted) {
        Ok(new_task) => new_task,
        Err(err) => return Err(err),
    };

    let success = format!("Task '{}' added!", new_task.desc.clone());

    tasks.push(new_task);

    Ok(success)
}


// Removes a task from a tasks vec
pub fn remove_task(tasks: &mut Vec<Task>, task_index: usize) {
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
    let new_task = match Task::build(task.list.clone(), new_desc, task.status.clone()) {
        Ok(new_task) => new_task,
        Err(err) => return Err(err),
    };

    print_success(format!("Updated Task to '{}'", &new_task.desc).as_str());

    Ok(new_task)
}


// Function to sort tasks from completed to not started
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


