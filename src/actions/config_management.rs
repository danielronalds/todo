use crate::user_config::UserConfig;
use crate::print_success;


// Function to set the current tasklist
pub fn set_current_tasklist(users_config: &mut UserConfig, new_tasklist: String) 
    -> Result<(), &'static str> {
    if new_tasklist.is_empty() {
        return Err("No tasklist name supplied!");
    }

    for list_name in &users_config.tasklists {
        if &new_tasklist == list_name {
            users_config.current_list = new_tasklist;

            print_success(format!("Switched to {} list!", users_config.current_list).as_str());

            return Ok(());
        }
    }

    Err("Tasklist doesn't exist!")
}


// Function to update the smart Id option
pub fn set_smart_id(users_config: &mut UserConfig, value: &str) -> Result<(), &'static str> {
    match value {
        "true" => {
            users_config.smart_id = true;
            print_success(
                format!("Task Id's will now show when there are {} or more tasks!",
                        users_config.smart_id_num).as_str()
            );
        },

        "false" => {
            users_config.smart_id = false;
            print_success("Disabled smart id!")
        },

        _ => return Err("Invalid option, Must be either true or false!"), 
    };
    
    Ok(())
}


// Function to set the number of tasks for smart id to trigger
pub fn set_num_of_tasks(users_config: &mut UserConfig, value: &str) -> Result<(), &'static str> {
    // Parses the value given, returning an error if the parse failed
    let num_of_tasks = match value.parse() {
        Ok(num) => num,
        Err(_) => return Err("Incorrect value, must be an integar!"),
    };

    users_config.smart_id_num = num_of_tasks;

    print_success(format!("Task Id's will now show up when there are {} or more Tasks!", 
                  users_config.smart_id_num).as_str());

    Ok(())
}


// Function to update the always show id option
pub fn set_always_show_id(users_config: &mut UserConfig, value: &str) -> Result<(), &'static str> {
    match value {
        "true" => {
            users_config.always_show_id = true;
            print_success("Enabled always showing Task Id's!")
        },

        "false" => {
            users_config.always_show_id = false;
            print_success("Disabled always showing Task Id's!")
        },

        _ => return Err("Invalid option, Must be either true or false!"), 
    };
    
    Ok(())
}
