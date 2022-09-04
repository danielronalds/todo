// Declaring Modules
mod actions;

mod task;

mod user_config;

// Use statments
use std::process::exit;

use actions::task_management;
use actions::file_management;
use actions::config_management;

use colored::Colorize;
use task::Task;
use user_config::UserConfig;


pub struct Config {
    pub command: String,
    pub command_arg: String,
    pub second_arg: String,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, &'static str> {
        // Checks to see if any command was supplied
        if args.len() < 2  {
            return Err("No command supplied!");
        }

        // Setting the arguments
        let command = args[1].clone();
        let mut command_arg = String::new();
        let mut second_arg = String::new();

        // Checks to see if a command argument was supplied, setting it if it was 
        if args.len() > 2 {
            command_arg = args[2].clone();
        } 

        if args.len() > 3 {
            second_arg = args[3].clone();
        }

        Ok(Config {
            command,
            command_arg,
            second_arg,
        })
    }

    pub fn argument_to_index(&self) -> usize {
        self.command_arg.parse().unwrap_or_else(|_| {
            print_error("That is not a valid Task ID!");
            exit(1);
        })
    }
}


// Function to check if a task exists in the tasklist
fn task_exists(task_index: usize, tasklist: &Vec<Task>) {
    if task_index >= tasklist.len() {
        print_error("Task does not exist!");
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


// Function to print errors
pub fn print_error(message: &str) {
    let error_symbol = format!("[{}]", "!".red());

    eprint!("{} {}", error_symbol.bold(), message);
}


// Function to print success messages
fn print_success(message: &str) {
    let success_symbol = format!("[{}]", "!".blue());

    eprint!("{} {}\n", success_symbol.bold(), message);
}


// Main run function
pub fn run(config: Config) {
    // Runs the commands that do not require a taskslist 
    match config.command.as_str() {
        // Init command
        "init" => {
            file_management::init_list().unwrap_or_else(|err| {
                print_error(format!("{}", err).as_str());
            });
            exit(1);
        },

        "help" => {
            actions::show_help();
            exit(1);
        },

        "version" => {
            actions::show_version();
            exit(1);
        },

        // Continues the code if the command requires a tasklist
        _ => (),
    }
    
    // Open the tasks file, exiting the program with an error message if the file fails to open
    let read_file = file_management::read_task_list().unwrap_or_else(|err| {
        print_error(err);
        exit(1);
    });
    
    let mut users_config = read_file.1;

    // Getting the current task list from the config
    let current_list = users_config.current_list.clone();

    let sorted_tasks = task_management::sort_by_current_list(current_list.clone(), read_file.0);

    let mut tasks = sorted_tasks.0; 

    match config.command.as_str() {
        // Deletes the tasklist in the directory
        "deletelist" => {
            file_management::delete_list().unwrap_or_else(|err| {
                print_error(err);
            });

            exit(1);
        },

        // Changes the current tasklist
        "tasklist" => {
            let new_list = config.command_arg;
            match config_management::set_current_tasklist(&mut users_config, new_list) {
                Ok(_) => (),
                Err(err) => print_error(err),
            }
        }

        // Lists the stored tasklists
        "tasklists" => task_management::list_tasklists(&users_config),

        // Creates a new tasklist
        "new" => {
            let new_list_name = config.command_arg;
            task_management::add_tasklist(&mut users_config, new_list_name).unwrap_or_else(|err| {
                print_error(err);
                exit(1);
            })
        }

        // List the current tasks
        "list" => task_management::list_tasks(&tasks, &users_config),

        // Add a task
        "add" => {
            let task_desc = config.command_arg;
            task_management::add_task(&mut tasks, task_desc, current_list).unwrap_or_else(|err| {
                print_error(err);
                exit(1);
            });
        },

        // Delete a task
        "delete" | "remove" => {
            let task_index = get_task_index(config, &tasks);

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

        // Update a tasks description 
        "update" => {
            let new_desc = config.second_arg.clone();

            let task_index = get_task_index(config, &tasks); 

            tasks[task_index] = task_management::update_task(&tasks[task_index], new_desc)
                .unwrap_or_else(|err| {
                    print_error(err);
                    exit(1);
                });
        }

        // Sort the task list
        "sort" => {
            // Sorting the task list
            tasks = task_management::sort_tasks(tasks);

            // Displaying the sorted list
            task_management::list_tasks(&tasks, &users_config);
        }

        // Remove completed tasks
        "cleanup" => {
            // Removing completed tasks
            task_management::cleanup_list(&mut tasks);

            println!("");
            
            // Displaying the cleaned up task list
            task_management::list_tasks(&tasks, &users_config);
        }

        "set" => config_command_management(&config, &mut users_config),

        // If the user has not typed a valid command, inform them
        _ => {
            print_error("Unrecognised command");
            println!("");
            actions::show_help();
        },
    }

    // Combined tasklist
    // ROUGH CODE
    let mut all_tasks: Vec<Task> = Vec::new();
    for task in tasks {
        all_tasks.push(task);
    }
    for task in sorted_tasks.1 {
        all_tasks.push(task);
    }

    file_management::save_task_list(all_tasks, users_config).unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}


// Function to manage the set command
fn config_command_management(config: &Config, users_config: &mut UserConfig) {
    match config.command_arg.as_str() {
        "smart_id" => {
            config_management::set_smart_id(users_config, &config.second_arg)
                .unwrap_or_else(|err| {
                    print_error(err);
                    exit(1);
                })
        }

        "num_of_tasks" => {
            config_management::set_num_of_tasks(users_config, &config.second_arg)
                .unwrap_or_else(|err| {
                    print_error(err);
                    exit(1);
                })
        }

        "always_show_id" => {
            config_management::set_always_show_id(users_config, &config.second_arg)
                .unwrap_or_else(|err| {
                    print_error(err);
                    exit(1);
                })
        }

        _ => {
            print_error("Unrecognised command");
        }
    }
}
