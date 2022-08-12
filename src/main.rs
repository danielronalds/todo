mod actions;

mod task;

fn main() {
    actions::file_management::init_list();

    let mut tasks = actions::file_management::read_task_list();

    actions::task_management::list_tasks(&tasks);

    actions::task_management::remove_task(&mut tasks, 0);

    actions::file_management::save_task_list(tasks);
}
