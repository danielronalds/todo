use std::process;

use clap::Parser;

use todo::args;
use todo::args::TodoArgs;

use todo::task::Task;

fn main() {
    let args = TodoArgs::parse();

    let mut tasks_vec: Vec<Task> = match todo::read_tasks_file() {
        Ok(tasks_vec) => tasks_vec,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut config = todo::read_config_file();

    match args.command {
        args::Commands::Tasks => match todo::list_tasks(&tasks_vec) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        },

        args::Commands::Sort => match todo::sort_list(&mut tasks_vec) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        },

        args::Commands::Add(arguments) => {
            match todo::new_task(arguments) {
                Ok(task) => tasks_vec.push(task),
                Err(err) => eprintln!("{}", err),
            };
        }

        args::Commands::Delete(arguments) => {
            println!("{}", todo::delete_task(&mut tasks_vec, arguments));
        }

        args::Commands::Start(arguments) => {
            println!("{}", todo::start_task(&mut tasks_vec, arguments));
        }

        args::Commands::Finish(arguments) => {
            println!("{}", todo::finish_task(&mut tasks_vec, arguments));
        }

        args::Commands::Restart(arguments) => {
            println!("{}", todo::restart_task(&mut tasks_vec, arguments));
        }

        args::Commands::Update(arguments) => {
            println!("{}", todo::update_task(&mut tasks_vec, arguments));
        }

        args::Commands::List(arguments) => {
            println!("{}", todo::manage_lists(&mut config, arguments))
        }
    }

    // Writing to the tasks file
    if let Err(err) = todo::write_tasks_file(tasks_vec) {
        println!("{}", err);
    }

    // Writing to the config file
    if let Err(err) = todo::write_config_file(config) {
        println!("{}", err);
    }
}
