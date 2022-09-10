use crate::user_config::UserConfig;


// Function to set the current tasklist
pub fn set_current_tasklist(users_config: &mut UserConfig, new_tasklist: String) 
    -> Result<String, &'static str> {
    if new_tasklist.is_empty() {
        return Err("No tasklist name supplied!");
    }

    // Checks to see if the tasklist exists, setting the current tasklist if it does
    for list_name in &users_config.tasklists {
        if &new_tasklist == list_name {
            users_config.current_list = new_tasklist;

            let message = format!("Switched to {} list!", users_config.current_list);

            return Ok(message);
        }
    }

    Err("Tasklist doesn't exist!")
}


// Function to update the smart Id option
pub fn set_smart_id(users_config: &mut UserConfig, value: &str) -> Result<String, &'static str> {
    match value {
        "true" => {
            users_config.smart_id = true;
                return Ok(format!("Task Id's will now show when there are {} or more tasks!", 
                           users_config.smart_id_num))
        },

        "false" => {
            users_config.smart_id = false;
            return Ok("Disabled smart id!".to_string());
        },

        _ => return Err("Invalid option, Must be either true or false!"), 
    };
}


// Function to set the number of tasks for smart id to trigger
pub fn set_num_of_tasks(users_config: &mut UserConfig, value: &str) 
    -> Result<String, &'static str> {
    // Parses the value given, returning an error if the parse failed
    let num_of_tasks = match value.parse() {
        Ok(num) => num,
        Err(_) => return Err("Incorrect value, must be an integar!"),
    };

    users_config.smart_id_num = num_of_tasks;

    Ok(format!("Task Id's will now show up when there are {} or more Tasks!", 
                  users_config.smart_id_num))
}


// Function to update the always show id option
pub fn set_always_show_id(users_config: &mut UserConfig, value: &str) 
    -> Result<String, &'static str> {
    match value {
        "true" => {
            users_config.always_show_id = true;
            return Ok("Enabled always showing Task Id's!".to_string())
        },

        "false" => {
            users_config.always_show_id = false;
            return Ok("Disabled always showing Task Id's!".to_string())
        },

        _ => return Err("Invalid option, Must be either true or false!"), 
    };
}