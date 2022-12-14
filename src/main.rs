use std::process;

use clap::Parser;

use todo::args;
use todo::args::TodoArgs;

use todo::task::Task;

fn main() {
    let args = TodoArgs::parse();

    let unfiltered_tasks_vec: Vec<Task> = match todo::read_tasks_file() {
        Ok(tasks_vec) => tasks_vec,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };

    let mut config = todo::read_config_file();

    // Filtering the tasks vec so that only the current list is worked on
    let filtered_vecs = todo::filter_task_vec(unfiltered_tasks_vec, &config);

    let mut tasks_vec = filtered_vecs.0;

    let mut other_tasks = filtered_vecs.1;

    match args.command {
        args::Commands::Tasks(arguments) => {
            match todo::list_tasks(&mut tasks_vec, &other_tasks, &config, arguments) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err),
            }
        }

        args::Commands::Sort => match todo::sort_list(&mut tasks_vec) {
            Ok(_) => (),
            Err(err) => eprintln!("{}", err),
        },

        args::Commands::Cleanup(arguments) => println!(
            "{}",
            todo::cleanup_list(&mut tasks_vec, &mut other_tasks, arguments)
        ),

        args::Commands::Add(arguments) => {
            match todo::new_task(arguments, &config) {
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
            println!("{}", todo::manage_lists(&mut config, arguments));
        }

        args::Commands::Config(arguments) => {
            println!("{}", todo::manage_config(&mut config, arguments));
        }

        args::Commands::Nuke => {
            println!("{}", todo::nuke_todo());
            // Exiting the program so that it doesn't attempt to serialise the program state
            process::exit(1);
        }
    }

    // Adding the other tasks back into the tasks_vec
    tasks_vec.extend(other_tasks);

    // Ensuring all tasks belong to a valid list
    tasks_vec.retain(|task| config.is_valid_list(&task.list()));

    // Writing to the tasks file
    if let Err(err) = todo::write_tasks_file(tasks_vec) {
        println!("{}", err);
    }

    // Writing to the config file
    if let Err(err) = todo::write_config_file(config) {
        println!("{}", err);
    }
}
