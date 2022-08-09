use task::Task;

mod actions;

mod task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    actions::add_task(&mut tasks, String::from("Just a basic task"));
    actions::add_task(&mut tasks, String::from("Just another basic task"));
    actions::add_task(&mut tasks, String::from("Yet another basic task"));
    
    actions::start_task(&mut tasks[0]);

    actions::finish_task(&mut tasks[1]);

    actions::remove_task(&mut tasks, 3);

    actions::list_tasks(&tasks);
}
