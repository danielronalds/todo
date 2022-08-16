use std::{env, process};

mod actions;

mod task;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    
    run(config);
}


struct Config {
    command: String,
    command_arg: String,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, &'static str> {
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
}


fn run(config: Config) {
    // To prevent accessing the task list if it doesn't exit
    if config.command.as_str() == "init" {
        actions::file_management::init_list();
        process::exit(1);
    }
    
    let mut tasks = actions::file_management::read_task_list();

    // This is shit, hobbled together code. However for now it works, which means
    // this program by bare definition, does something!
    match config.command.as_str() {
        // List the current tasks
        "list" => actions::task_management::list_tasks(&tasks),
        // Add a task
        "add" => {
            let task_desc = config.command_arg;
            actions::task_management::add_task(&mut tasks, task_desc);
        },
        // Delete a task
        "delete" => {
            let task_index = config.command_arg.parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            actions::task_management::remove_task(&mut tasks, task_index);
        },
        // Start a task
        "start" => {
            // Parsing second argument into an index for accessing the task vec
            let task_index: usize = config.command_arg.parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            // Check to see if the task exists
            if task_index > tasks.len() {
                eprint!("Task does not exist!");
                process::exit(1);
            }
            actions::task_management::start_task(&mut tasks[task_index]);
        },
        // Finish a task
        "finish" => {
            // Parsing second argument into an index for accessing the task vec
            let task_index: usize = config.command_arg.parse().unwrap_or_else(|_| {
                eprintln!("That is not a valid Task ID!");
                process::exit(1);
            });
            // Check to see if the task exists
            if task_index > tasks.len() {
                eprint!("Task does not exist!");
                process::exit(1);
            }
            actions::task_management::finish_task(&mut tasks[task_index]);
        },
        // If the user has not typed a valid command, inform them
        _ => eprintln!("Unrecognised command!"),
    }

    actions::file_management::save_task_list(tasks);
}
