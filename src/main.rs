use task::Task;

mod actions;

mod task;

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    actions::task_management::add_task(&mut tasks, String::from("Just a basic task"));
    actions::task_management::add_task(&mut tasks, String::from("Another basic task"));
    actions::task_management::add_task(&mut tasks, String::from("Yet another basic task"));

    actions::task_management::list_tasks(&tasks);

    actions::task_management::start_task(&mut tasks[0]);

    actions::task_management::finish_task(&mut tasks[1]);

    actions::task_management::remove_task(&mut tasks, 2);
    
    actions::task_management::remove_task(&mut tasks, 3);

    actions::file_management::init_list();
}
