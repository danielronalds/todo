pub struct UserConfig {
    pub always_show_id: bool,
    pub smart_id: bool,
    pub smart_id_num: u8, 
}

impl UserConfig {
    // Method to build a user config
    pub fn build(args: Vec<String>) -> Result<UserConfig, &'static str>{
        // Check if the vec contains the right data
        if args.len() != 3 {
            return Err("Wrong number of arguments!");
        }

        // Retrieving bool values from String's
        let always_show_id_value = match args[0].as_str() {
            "true" => true,
            "false" => false,
            // Might change this to later default to a value, same as below. I'm not sure how I'm 
            // going to handle errors yet
            _ => return Err("Incorrect value for always_show_id option!"),
        };

        let smart_id_value = match args[1].as_str() {
            "true" => true,
            "false" => false,
            _ => return Err("Incorrect value for always_show_id option!"),
        };

        // Attempts to parse the string to an int returning a default of ↓ if there is an error
        let smart_id_num_value: u8 = args[2].parse().unwrap_or_else(|_| {5});

        Ok(UserConfig {
            always_show_id: always_show_id_value,
            smart_id: smart_id_value,
            smart_id_num: smart_id_num_value,
        })
    }

    pub fn to_save_format(&self) -> String {
        format!("{}|{}|{}", &self.always_show_id, &self.smart_id, &self.smart_id_num)
    }
}
