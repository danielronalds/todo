use clap::Parser;

use todo::args;
use todo::args::TodoArgs;

use todo::task::Task;

fn main() {
    let args = TodoArgs::parse();

    let mut tasks_vec: Vec<Task> = vec![
        Task::new(String::from("A basic task!"), todo::task::TaskStatus::NotStarted).unwrap(),
        Task::new(String::from("Another basic task!"), todo::task::TaskStatus::NotStarted).unwrap()
    ];

    match args.command {
        args::Commands::Add(arguments) => {
            match todo::new_task(arguments) {
                Ok(task) => tasks_vec.push(task),
                Err(err) => println!("{}", err),
            };
        }
    }

    println!("{:?}", tasks_vec);
}
