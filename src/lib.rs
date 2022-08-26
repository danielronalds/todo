// Declaring Modules
mod actions;

mod task;

// Use statments
use std::process::exit;

use actions::task_management;
use actions::file_management;

use task::Task;


pub struct Config {
    pub command: String,
    pub command_arg: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        // Checks to see if any command was supplied
        if args.len() < 2  {
            return Err("No command supplied!");
        }

        // Setting the arguments
        let cmd = args[1].clone();
        let mut cmd_arg = String::new();

        // Checks to see if a command argument was supplied, setting it if it was 
        if args.len() > 2 {
            cmd_arg = args[2].clone();
        } 

        Ok(Config {
            command: cmd,
            command_arg: cmd_arg,
        })
    }

    pub fn argument_to_index(&self) -> usize {
        self.command_arg.parse().unwrap_or_else(|_| {
            eprintln!("That is not a valid Task ID!");
            exit(1);
        })
    }
}


// Function to check if a task exists in the tasklist
fn task_exists(task_index: usize, tasklist: &Vec<Task>) {
    if task_index >= tasklist.len() {
        eprint!("Task does not exist!");
        exit(1);
    }
}


// Function to get the task's id
fn get_task_index(config: Config, tasklist: &Vec<Task>) -> usize {
    // Getting the task's index
    let mut task_index = config.argument_to_index();

    task_index -= 1;

    // Check to see if the task exists
    task_exists(task_index, tasklist);

    task_index
}


// Main run function
pub fn run(config: Config) {
    // Checks to see if the user is trying to create a task list before attempting to open one, 
    // to prevent the user from never being able to create a tasks list, as when the function
    // read_task_list() returns an Err() the program exists.
    if config.command.as_str() == "init" {
        file_management::init_list().unwrap_or_else(|err| {
            eprintln!("{}", err)
        });
        exit(1);
    }
    
    // Open the tasks file, exiting the program with an error message if the file fails to open
    let mut tasks = file_management::read_task_list().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    match config.command.as_str() {
        "help" => actions::show_help(),

        // List the current tasks
        "list" => task_management::list_tasks(&tasks),

        // Add a task
        "add" => {
            let task_desc = config.command_arg;
            task_management::add_task(&mut tasks, task_desc).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1);
            });
        },

        // Delete a task
        "delete" | "remove" => {
            let task_index = config.argument_to_index();
            task_management::remove_task(&mut tasks, task_index);
        },

        // Start a task
        "start" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            task_management::start_task(&mut tasks[task_index]);
        },

        // Finish a task
        "finish" | "tick" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            task_management::finish_task(&mut tasks[task_index]);
        },

        // Restart a task
        "restart" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            task_management::restart_task(&mut tasks[task_index]);
        }

        // Sort the task list
        "sort" => {
            // Sorting the task list
            tasks = task_management::sort_tasks(tasks);

            // Displaying the sorted list
            task_management::list_tasks(&tasks);
        }

        // Remove completed tasks
        "cleanup" => {
            task_management::cleanup_list(&mut tasks);

            task_management::list_tasks(&tasks);
        }

        // If the user has not typed a valid command, inform them
        _ => eprintln!("Unrecognised command, try help to see the list of commands!"),
    }

    file_management::save_task_list(tasks).unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}
