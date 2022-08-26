pub mod task_management;
pub mod file_management;

// Function to list all the commands
pub fn show_help() {
    println!("Todo v{}, developed by Daniel Ronalds", env!("CARGO_PKG_VERSION"));
    println!("");
    println!("usage: <COMMAND> <ARGUMENT>");
    println!("");
    println!("  init: Creates a tasklist in the current directory");
    println!("");
    println!("  list: Shows all the tasks on the tasks list");
    println!("");
    println!("  Sort: Sorts the task list so that all tasks are ouputted in descending order");
    println!("        from Completed to Not Started");
    println!("");
    println!("  cleanup: Removes all Completed tasks from the task list");
    println!("");
    println!("  add <TASK_DESC>    Adds a task to the tasks list");
    println!("");
    println!("  delete <TASK_ID>   Deletes a task with the given id");
    println!("");
    println!("  start <TASK_ID>    Start a task with the given id");
    println!("");
    println!("  finish <TASK_ID>   Finish a task with the given id");
    println!("");
    println!("  restart <TASK_ID>  Restart a task with the given id");
    println!("");
}
