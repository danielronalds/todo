use task::Task;

mod actions;

mod task;

fn main() {
    let tasks = actions::file_management::read_task_list();

    actions::task_management::list_tasks(&tasks);
}
