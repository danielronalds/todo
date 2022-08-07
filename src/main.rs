use task::Task;

mod commands;

mod task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    commands::add_task(&mut tasks, String::from("Just a basic task"));
    commands::add_task(&mut tasks, String::from("Just another basic task"));
    commands::add_task(&mut tasks, String::from("Yet another basic task"));
    
    commands::start_task(&mut tasks[0]);

    commands::remove_task(&mut tasks, 3);

    commands::list_tasks(&tasks);
}
