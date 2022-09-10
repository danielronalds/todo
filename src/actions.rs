pub mod config_management;
pub mod file_management;
pub mod task_management;
pub mod tasklist_management;

// Function to list all the commands
pub fn show_help() {
    show_version();
    println!("");
    println!("usage: todo <COMMAND> <ARGUMENT> <SECOND_ARGUMENT>");
    println!("");
    println!("  help                           Show this dialog");
    println!("");
    println!("  version                        Prints the version of todo installed");
    println!("");
    println!("  init                           Creates a tasklist in the current");
    println!("                                 directory");
    println!("");
    println!("  deletelist                     Deletes the todo instance in the current");
    println!("                                 directory ");
    println!("                                 WARNING: This action is permanent");
    println!("");
    println!("  list                           Shows all the tasks on the tasks list");
    println!("");
    println!("  sort                           Sorts the tasklist in order from");
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
    println!("  tasklists                      List all the current tasklists");
    println!("");
    show_tasklist_help(false);
    println!("");
    show_config_help(false);
}

// The help meny for tasklist management
pub fn show_tasklist_help(header: bool) {
    // The header is different depending on whether its called from the help command or the 
    // tasklist help command
    if header {
        println!("usage: todo tasklist <OPTION> <VALUE>");
    } else {
        println!("  tasklist <OPTION> <VALUE>      Manage tasklists:");
    }
    println!("");
    println!("     new <NEW_TASKLIST_NAME>     Creates a new tasklist with the given");
    println!("                                 name");
    println!("");
    println!("     set <TASKLIST_NAME>         Changes the current tasklist to the");
    println!("                                 tasklist with the given name");
    println!("");
    println!("     update <TASKLIST_NAME>      Changes the current tasklist's name");
    println!("                                 to the given name");
    println!("");
    println!("     delete                      Deletes the current tasklist as well");
    println!("                                 as all the tasks listed under it.");
    println!("                                 WARNING: This action is permanent");
}

// The help menu for config management
pub fn show_config_help(header: bool) {
    // Same deal as the function above
    if header {
        println!("usage: todo set <CONFIG_OPTION> <VALUE>");
    } else {
        println!("  set <CONFIG_OPTION> <VALUE>    Change configuration options:");
    }
    println!("");
    println!("     show_list_name <true/false> Whether the current tasklists name");
    println!("                                 should be shown above the tasks printed");
    println!("                                 by the list command");
    println!("");
    println!("     always_show_id <true/false> Whether task id's should always be shown");
    println!("                                 regardless of how many tasks there are");
    println!("                                 Disabled by default");
    println!("");
    println!("     smart_id <true/false>       Only show task id's when there are a");
    println!("                                 certain number of tasks or more on the");
    println!("                                 task list. Enabled by default.");
    println!("                                 NOTE: Overridden by always_show_id");
    println!("");
    println!("     num_of_tasks <NUM_OF_TAKS>  The number of tasks to start displaying");
    println!("                                 task id's when smart_id is enabled.");
    println!("                                 Default is 5.");
}

// Function to show version
pub fn show_version() {
    println!("Todo v{}, developed by Daniel Ronalds", env!("CARGO_PKG_VERSION"));
}
