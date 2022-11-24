// Modules
mod args;
mod task;
mod commands;
mod task_management;

use clap::Parser;

use args::TodoArgs;

use task::Task;

/// The main run function of the library
///
/// This code handles parsing of the arguments and the main code calling
pub fn run() {
    let args = TodoArgs::parse();

    let mut tasks_vec: Vec<Task> = Vec::new();

    match args.command {
        args::Commands::Add(arguments) => {
            match commands::new_task(arguments) {
                Ok(task) => tasks_vec.push(task),
                Err(err) => println!("{}", err),
            };
        }
    }

    println!("{:?}", tasks_vec);
}
