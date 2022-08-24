pub mod task_management;
pub mod file_management;

// Function to list all the commands
pub fn show_help() {
    println!("usage: <COMMAND> <ARGUMENT>");
    println!("help: Shows this dialog");
    println!("init: Creates a tasklist in the current directory");
    println!("list: Shows all the tasks on the tasks list");
    println!("add <TASK_DESC>: Adds a task to the tasks list");
    println!("delete <TASK_ID>: Deletes a task with the given id");
    println!("start <TASK_ID>: Start a task with the given id");
    println!("finish <TASK_ID>: Finish a task with the given id\n");
}
