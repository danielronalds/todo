mod commands;

mod task;

fn main() {
    let task1 = task::Task {
       desc: String::from("Just a basic task"),
       status: task::TaskStatus::NotStarted,
    };

    let mut tasks = vec![task1];

    commands::start_task(&mut tasks[0]);

    commands::list_tasks(&tasks);

    commands::finish_task(&mut tasks[0]);

    commands::list_tasks(&tasks);
}
