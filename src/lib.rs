// Declaring Modules
mod actions;

mod task;

mod user_config;

// Use statments
use std::process::exit;

use actions::task_management;
use actions::file_management;
use actions::config_management;
use actions::tasklist_management;

use colored::Colorize;
use task::Task;
use user_config::UserConfig;


/// Struct for storing what paramaters the user ran the program with
pub struct Config {
    pub command: String,
    pub command_arg: String,
    pub second_arg: String,
}

impl Config {
    /// Builds a config struct with the parameters the user ran the program with
    ///
    /// Parameters
    /// args:     The arguments the user ran the program with
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

    /// Converts the command argument string to a usize
    pub fn argument_to_index(&self) -> usize {
        self.command_arg.parse().unwrap_or_else(|_| {
            print_error("That is not a valid Task ID!");
            exit(1);
        })
    }
}


/// Checks if a task exists in the given tasklist
///
/// Parameters
/// task_index: Index of the task
/// tasklist: The tasklist the task is in
fn task_exists(task_index: usize, tasklist: &Vec<Task>) {
    if task_index >= tasklist.len() {
        print_error("Task does not exist!");
        exit(1);
    }
}


/// Gets the current index of the given task from the config, checking if it exists
///
/// Parameters
/// config:     The config the user ran the program with
/// tasklist:   The tasklist the task is in
fn get_task_index(config: Config, tasklist: &Vec<Task>) -> usize {
    // Getting the task's index
    let mut task_index = config.argument_to_index();

    task_index -= 1;

    // Check to see if the task exists
    task_exists(task_index, tasklist);

    task_index
}


/// Prints an error message using eprint! with pretty formatting
pub fn print_error(message: &str) {
    let error_symbol = format!("[{}]", "!".bright_red());

    eprint!("{} {}", error_symbol.bold(), message);
}


/// Prints a success message using eprint! with pretty formatting
fn print_success(message: &str) {
    let success_symbol = format!("[{}]", "!".bright_blue());

    eprint!("{} {}\n", success_symbol.bold(), message);
}


/// The main logic of the program. Checks what commands the user ran the program with and calls the
/// appropriate functions
///
/// Parameters:
/// config: The parameteres the user passed when they ran the program
pub fn run(config: Config) {
    // Runs the commands that do not require a taskslist 
    match config.command.as_str() {
        // Init command
        "init" => {
            match file_management::init_list() {
                Ok(message) => print_success(message),
                Err(err) => print_error(err),
            };
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

    let sorted_tasks = tasklist_management::filter_list(current_list.clone(), read_file.0);

    let mut tasks = sorted_tasks.0; 

    let other_tasks = sorted_tasks.1;

    match config.command.as_str() {
        // Deletes the tasklist in the directory
        "deletelist" => {
            match file_management::delete_list() {
                Ok(message) => print_success(message),
                Err(err) => print_error(err),
            }
            exit(1);
        },

        // List the current tasks
        "list" => {
            task_management::list_tasks(&tasks, &users_config).unwrap_or_else(|err| {
                print_error(err);
            })
        },

        "listall" => {
            task_management::list_all_tasks(tasks.to_vec(), other_tasks.to_vec(), &users_config);
        }

        // Add a task
        "add" => {
            let task_desc = config.command_arg;
            match task_management::add_task(&mut tasks, task_desc, current_list) {
                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            };
        },

        // Delete a task
        "delete" | "remove" => {
            let task_index = get_task_index(config, &tasks);

            print_success(task_management::remove_task(&mut tasks, task_index).as_str());
        },

        // Start a task
        "start" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            match task_management::start_task(&mut tasks[task_index]) {
                Ok(message) => print_success(&message),
                Err(err) => print_error(err),
            };
        },

        // Finish a task
        "finish" | "tick" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            match task_management::finish_task(&mut tasks[task_index]) {
                Ok(message) => print_success(&message),
                Err(err) => print_error(err),
            };
        },

        // Restart a task
        "restart" => {
            // Getting task index
            let task_index = get_task_index(config, &tasks);
            
            // Updating task status
            print_success(task_management::restart_task(&mut tasks[task_index]).as_str());
        }

        // Update a tasks description 
        "update" => {
            let new_desc = config.second_arg.clone();

            let task_index = get_task_index(config, &tasks); 
            
            let updated_task = match task_management::update_task(&tasks[task_index], new_desc) {
                Ok(updated_task) => updated_task,
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            };

            // Updating the task
            tasks[task_index] = updated_task.0;

            print_success(updated_task.1.as_str());
        }

        // Sort the task list
        "sort" => {
            // Sorting the task list
            tasks = task_management::sort_tasks(tasks);

            // Displaying the sorted list
            task_management::list_tasks(&tasks, &users_config).unwrap_or_else(|err| {
                print_error(err);
            });
        }

        // Remove completed tasks
        "cleanup" => {
            // Removing completed tasks
            print_success(task_management::cleanup_list(&mut tasks).as_str());

            println!("");
            
            // Displaying the cleaned up task list
            task_management::list_tasks(&tasks, &users_config).unwrap_or_else(|err| {
                print_error(err);
            });
        }

        // Manages the set command
        "set" => config_command_management(&config, &mut users_config),

        // Manages the tasklist command (Everything to do with them)
        "tasklist" | "tasklists" => {
            tasklist_command_management(&config, &mut users_config, &mut tasks);
        }

        // If the user has not typed a valid command, inform them
        _ => {
            print_error("Unrecognised command, try 'help'!");
        },
    }

    // Combined tasklist
    // ROUGH CODE
    let mut all_tasks: Vec<Task> = Vec::new();
    for task in tasks {
        all_tasks.push(task);
    }
    for task in other_tasks {
        all_tasks.push(task);
    }

    file_management::save_task_list(all_tasks, users_config).unwrap_or_else(|err| {
        eprintln!("{}", err);
    });
}


/// Function that manages all the tasklist related commands
///
/// Parameters
/// config:         The config the user ran the program with
/// users_config:   The users settings
/// tasks:          The tasks in the current tasklist
fn tasklist_command_management(config: &Config, users_config: &mut UserConfig, 
                               tasks: &mut Vec<Task>) {
    // Lists all the current tasklists
    if config.command.as_str() == "tasklists" {
        tasklist_management::list_tasklists(&users_config);
        return
    }

    match config.command_arg.as_str() {
        // Creating a new tasklist
        "new" => {
            let new_list_name = config.second_arg.clone();
            match tasklist_management::add_tasklist(users_config, new_list_name) {
                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            };
        }

        // Sets the current tasklist
        "set" => {
            let new_list = config.second_arg.clone();
            match config_management::set_current_tasklist(users_config, new_list) {
                Ok(message) => print_success(&message),
                Err(err) => print_error(err),
            }
        }

        // Updates the current tasklists name 
        "update" => {
            let new_name = config.second_arg.clone();
            match tasklist_management::update_tasklist_name(users_config, tasks, new_name) {
                Ok(message) => print_success(message),
                Err(err) => print_error(err)
            }
        }

        // Deletes the current tasklist
        "delete" => {
            match tasklist_management::delete_tasklist(users_config, tasks) {
                Ok(message) => print_success(message),
                Err(err) => print_error(err),
            }
        }

        // Help command
        "help" => actions::show_tasklist_help(true),

        _ => {
            // Checks if the user didn't type anything after the command, suggests the help command
            if config.command_arg.is_empty() {
                print_error("Unrecognised command, try 'tasklist help'!");
                return;
            }

            // Otherwise the user might be trying to switch tasklists, so call the function with
            // the command_arg as the new_list
            let new_list = config.command_arg.clone();
            match config_management::set_current_tasklist(users_config, new_list) {
                Ok(message) => print_success(&message),
                Err(err) => print_error(err),
            }
        },
    }
}


/// Function that manages all the set command
///
/// Parameters
/// config:         The config the user ran the program with
/// users_config:   The users settings
fn config_command_management(config: &Config, users_config: &mut UserConfig) {
    match config.command_arg.as_str() {
        "smart_id" => {
            match config_management::set_smart_id(users_config, &config.second_arg) {
                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            }
        }

        "num_of_tasks" => {
            match config_management::set_num_of_tasks(users_config, &config.second_arg) {
                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            }
        }

        "show_list_name" => {
            match config_management::set_display_list_name(users_config, &config.second_arg) {
                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            }
        }

        "always_show_id" => {
            match config_management::set_always_show_id(users_config, &config.second_arg) {

                Ok(message) => print_success(&message),
                Err(err) => {
                    print_error(err);
                    exit(1);
                }
            }
        }

        "help" => actions::show_config_help(true),

        _ => print_error("Unrecognised command, try 'set help'!"),
    }
}
