// Modules
mod args;
mod task;
mod task_management;

use args::TodoArgs;
use clap::Parser;

use crate::task::Task;
use crate::task::TaskErrors;
use crate::task::TaskStatus;

pub fn run() {
    let args = TodoArgs::parse();

    match &args.command {
        args::Commands::Add(input) => {
            match Task::new(input.description.clone(), TaskStatus::NotStarted) {
                Ok(task) => println!("{}", task.to_save_string()),
                Err(err) => match err {
                    TaskErrors::EmptyDescription => {
                        println!("Tasks cannot have empty descriptions!")
                    }
                    TaskErrors::InvalidCharInDescription => {
                        println!("Tasks cannot have the | char!")
                    }
                },
            };
        }
    }
}
