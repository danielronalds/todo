use clap::Parser;

use todo::args;
use todo::args::TodoArgs;

use todo::task::Task;
use todo::serialization::SerializationErrors;

fn main() {
    let args = TodoArgs::parse();

    let mut tasks_vec: Vec<Task> = vec![
        Task::new(
            String::from("A basic task!"),
            todo::task::TaskStatus::Completed,
        )
        .unwrap(),
        Task::new(
            String::from("Another basic task!"),
            todo::task::TaskStatus::InProgress,
        )
        .unwrap(),
        Task::new(
            String::from("Yet another basic task!"),
            todo::task::TaskStatus::NotStarted,
        )
        .unwrap(),
    ];

    match args.command {
        args::Commands::Tasks => {
            match todo::list_tasks(&tasks_vec) {
                Ok(_) => (),
                Err(err) => eprintln!("{}", err),
            }
        }

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
    }

    match todo::serialization::writer(tasks_vec) {
        Ok(_) => (),
        Err(err) => match err {
            SerializationErrors::FailedToCreateWriter => println!("Failed to create the writer!"),
            SerializationErrors::FailedToSerialize => println!("Failed to serialiaze the data!"),
            SerializationErrors::FailedToFlush => println!("Could not flush!")
        }
    }

}
