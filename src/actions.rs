pub mod config_management;
pub mod file_management;
pub mod task_management;

// Function to list all the commands
pub fn show_help() {
    show_version();
    println!("");
    println!("usage: todo <COMMAND> <ARGUMENT> <SECOND_ARGUMENT>");
    println!("");
    println!("  init                           Creates a tasklist in the current");
    println!("                                 directory");
    println!("");
    println!("  deletelist                     Deletes the tasklist in the current");
    println!("                                 directory");
    println!("");
    println!("  list                           Shows all the tasks on the tasks list");
    println!("");
    println!("  Sort                           Sorts the tasklist in order from");
    println!("                                 Completed to Not Started tasks");
    println!("");
    println!("  cleanup                        Removes all Completed tasks");
    println!("");
    println!("  add <TASK_DESC>                Adds a task to the tasks list");
    println!("");
    println!("  remove <TASK_ID>               Deletes a task with the given id");
    println!("");
    println!("  start <TASK_ID>                Start a task with the given id");
    println!("");
    println!("  finish <TASK_ID>               Finish a task with the given id");
    println!("");
    println!("  restart <TASK_ID>              Restart a task with the given id");
    println!("");
    println!("  update <TASK_ID> <NEW_DESC>    Update an existing tasks description");
    println!("");
    println!("  set <CONFIG_OPTION> <VALUE>    Update an existing tasks description");
}

// Function to show version
pub fn show_version() {
    println!("Todo v{}, developed by Daniel Ronalds", env!("CARGO_PKG_VERSION"));
}
