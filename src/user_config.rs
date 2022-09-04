pub struct UserConfig {
    pub always_show_id: bool,
    pub smart_id: bool,
    pub smart_id_num: u8, 
    pub current_list: String,
}

impl UserConfig {
    // Function to build a user config
    pub fn build(args: Vec<String>) -> UserConfig {
        // Check if the vec contains the right data, if not returns a default config
        if args.len() != 4 {
            return Self::default();
        }

        // Retrieving bool values from String's, with defaults
        let always_show_id = match args[0].as_str() {
            "true" => true,
            "false" => false,
            _ => false,
        };

        let smart_id = match args[1].as_str() {
            "true" => true,
            "false" => false,
            _ => true,
        };

        // Attempts to parse the string to an int, returning this  ↓ if there is an error
        let smart_id_num: u8 = args[2].parse().unwrap_or_else(|_| {5});

        let current_list = args[3].clone();

        UserConfig {
            always_show_id,
            smart_id,
            smart_id_num,
            current_list,
        }
    }

    // Function that returns a default user config
    pub fn default() -> UserConfig {
        UserConfig {
            always_show_id: false,
            smart_id: true,
            smart_id_num: 5,
            current_list: String::from("Main"),
        }
    }

    pub fn to_save_format(&self) -> String {
        format!("{}|{}|{}|{}\n", 
                &self.always_show_id, 
                &self.smart_id, 
                &self.smart_id_num,
                &self.current_list)
    }
}
