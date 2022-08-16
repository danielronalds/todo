use std::{env, process};

mod actions;

mod task;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command;

    // Checks to see if an argument has been provided or not
    if args.len() > 1 {
        command = args[1].as_str();
    } else {
        eprint!("No command!");
        process::exit(1);
    };


    // This is shit, hobbled together code. However for now it works, which means
    // this program by bare definition, does something!
    match command {
        // Init a task list
        "init" => actions::file_management::init_list(),
        // List the current tasks
        "list" => {
            let tasks = actions::file_management::read_task_list();
            actions::task_management::list_tasks(&tasks);
            actions::file_management::save_task_list(tasks);
        },
        // Add a task
        "add" => {
            let mut tasks = actions::file_management::read_task_list();
            let sub_command = args[2].as_str();
            actions::task_management::add_task(&mut tasks, sub_command.to_string());
            actions::file_management::save_task_list(tasks);
        },
        // Delete a task
        "delete" => {
            let mut tasks = actions::file_management::read_task_list();
            let task_index = args[2].parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            actions::task_management::remove_task(&mut tasks, task_index);
            actions::file_management::save_task_list(tasks);
        },
        // Start a task
        "start" => {
            let mut tasks = actions::file_management::read_task_list();
            // Parsing second argument into an index for accessing the task vec
            let task_index: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            // Check to see if the task exists
            if task_index > tasks.len() {
                eprint!("Task does not exist!");
                process::exit(1);
            }
            actions::task_management::start_task(&mut tasks[task_index]);
            actions::file_management::save_task_list(tasks);
        },
        // Finish a task
        "finish" => {
            let mut tasks = actions::file_management::read_task_list();
            // Parsing second argument into an index for accessing the task vec
            let task_index: usize = args[2].parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            // Check to see if the task exists
            if task_index > tasks.len() {
                eprint!("Task does not exist!");
                process::exit(1);
            }
            actions::task_management::finish_task(&mut tasks[task_index]);
            actions::file_management::save_task_list(tasks);
        },
        // If the user has not typed a valid command, inform them
        _ => eprintln!("Unrecognised command!"),
    }

}
