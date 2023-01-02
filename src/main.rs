use std::process;

use clap::Parser;

use todo::args;
use todo::args::TodoArgs;

use todo::task::Task;

use todo::print_info;

fn main() {
    let args = TodoArgs::parse();

    let unfiltered_tasks_vec: Vec<Task> = match todo::read_tasks_file() {
        Ok(tasks_vec) => tasks_vec,
        Err(err) => {
            print_info(err);
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
                // List is already formated so print it as normal
                Ok(list) => println!("{}", &list),
                Err(err) => print_info(err),
            }
        }

        args::Commands::Cleanup(arguments) => {
            if config.command_feedback() {
                print_info(todo::cleanup_list(
                    &mut tasks_vec,
                    &mut other_tasks,
                    arguments,
                ))
            }
        }

        args::Commands::Add(arguments) => {
            // If the user provides a description, add the task with that description
            if let Some(description) = arguments.description {
                match todo::new_task(description, &config) {
                    Ok(task) => {
                        tasks_vec.push(task);
                        if config.command_feedback() {
                            print_info("Task added!")
                        }
                    }
                    Err(err) => print_info(err),
                };
            } else {
                // Else enter add_mode
                todo::add_mode(&mut tasks_vec, &config);
            }
        }

        args::Commands::Delete(arguments) => {
            let message = todo::delete_task(&mut tasks_vec, arguments);
            if config.command_feedback() {
                print_info(message);
            }
        }

        args::Commands::Start(arguments) => {
            let message = todo::start_task(&mut tasks_vec, arguments);
            if config.command_feedback() {
                print_info(message);
            }
        }

        args::Commands::Finish(arguments) => {
            let message = todo::finish_task(&mut tasks_vec, arguments);
            if config.command_feedback() {
                print_info(message);
            }
        }

        args::Commands::Restart(arguments) => {
            let message = todo::restart_task(&mut tasks_vec, arguments);
            if config.command_feedback() {
                print_info(message);
            }
        }

        args::Commands::Update(arguments) => {
            let message = todo::update_task(&mut tasks_vec, arguments);
            if config.command_feedback() {
                print_info(message);
            }
        }

        args::Commands::List(arguments) => {
            if let Some(message) = todo::manage_lists(&mut config, arguments) {
                println!("{}", message);
            }
        }

        args::Commands::Config(arguments) => {
            // This will also be shown, so that the user knows what is happening with their config
            println!("{}", todo::manage_config(&mut config, arguments));
        }

        args::Commands::Nuke => {
            // This will be printed regardless of the config option as the user should always know
            // that this command worked
            print_info(todo::nuke_todo());
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
        print_info(err);
    }

    // Writing to the config file
    if let Err(err) = todo::write_config_file(config) {
        print_info(err);
    }
}
