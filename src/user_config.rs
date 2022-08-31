pub struct UserConfig {
    pub always_show_id: bool,
    pub smart_id: bool,
    pub smart_id_num: u8, 
}

impl UserConfig {
    // Function to build a user config
    pub fn build(args: Vec<String>) -> UserConfig {
        // Check if the vec contains the right data, if not returns a default config
        if args.len() != 3 {
            return Self::default();
        }

        // Retrieving bool values from String's, with defaults
        let always_show_id_value = match args[0].as_str() {
            "true" => true,
            "false" => false,
            _ => false,
        };

        let smart_id_value = match args[1].as_str() {
            "true" => true,
            "false" => false,
            _ => true,
        };

        // Attempts to parse the string to an int returning a default of ↓ if there is an error
        let smart_id_num_value: u8 = args[2].parse().unwrap_or_else(|_| {5});

        UserConfig {
            always_show_id: always_show_id_value,
            smart_id: smart_id_value,
            smart_id_num: smart_id_num_value,
        }
    }

    // Function that returns a default user config
    pub fn default() -> UserConfig {
        UserConfig {
            always_show_id: false,
            smart_id: true,
            smart_id_num: 5,
        }
    }

    pub fn to_save_format(&self) -> String {
        format!("{}|{}|{}", &self.always_show_id, &self.smart_id, &self.smart_id_num)
    }
}
