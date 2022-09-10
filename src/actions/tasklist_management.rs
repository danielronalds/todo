use colored::Colorize;

use crate::user_config::UserConfig;
use crate::task::Task;

use crate::print_error;


// Function to sort a tasklist and return a vec containing the tasks tagged with the current list
// as well as vec containing the rest
pub fn filter_list(current_list: String, tasks: Vec<Task>) -> (Vec<Task>, Vec<Task>) {
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
            return Err("A Tasklist with that name already exists!");
        }
    }

    let message = format!("Tasklist {} Added!", &new_tasklist);

    users_config.tasklists.push(new_tasklist);

    Ok(message)
}


// Function to update the current tasklists name
pub fn update_tasklist_name(users_config: &mut UserConfig, tasks: &mut Vec<Task>, new_name: String) 
    -> Result<&'static str, &'static str> {
    // Checking if the new name is empty, whether a tasklist wih it already exists or if contains 
    // illegal characters
    if new_name.is_empty() {
        return Err("No tasklist name supplied!");
    }
    
    if new_name.contains("|") {
        return Err("Tasklist names cannot contain the | character");
    }
    
    for list_name in &users_config.tasklists {
        if &new_name == list_name {
            return Err("A Tasklist with that name already exists!");
        }
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

