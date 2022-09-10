pub struct UserConfig {
    pub always_show_id: bool,
    pub smart_id: bool,
    pub smart_id_num: u8, 
    pub current_list: String,
    pub display_list_name: bool,
    pub tasklists: Vec<String>,
}

impl UserConfig {
    // Function to build a user config
    pub fn build(args: Vec<String>) -> UserConfig {
        // Check if the vec contains the right data, if not returns a default config
        if args.len() != 5 {
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
        
        let display_list_name = match args[4].as_str() {
            "true" => true,
            "false" => false,
            _ => false,
        };

        // Attempts to parse the string to an int, returning this  ↓ if there is an error
        let smart_id_num: u8 = args[2].parse().unwrap_or_else(|_| {5});

        let current_list = args[3].clone();

        let tasklists: Vec<String> = Vec::new();

        UserConfig {
            always_show_id,
            smart_id,
            smart_id_num,
            current_list,
            tasklists,
            display_list_name,
        }
    }

    // Function that returns a default user config
    pub fn default() -> UserConfig {
        let mut tasklists: Vec<String> = Vec::new();

        tasklists.push("Default".to_string());

        UserConfig {
            always_show_id: false,
            smart_id: true,
            smart_id_num: 5,
            current_list: String::from("Default"),
            tasklists,
            display_list_name: false,
        }
    }

    pub fn to_save_format(&self) -> String {
        format!("{}|{}|{}|{}|{}\n", 
                &self.always_show_id, 
                &self.smart_id, 
                &self.smart_id_num,
                &self.current_list,
                &self.display_list_name)
    }

    pub fn tasklists_to_save_format(&self) -> String {
        let mut saved_lists = String::new();

        let last_list = &self.tasklists.len() - 1;
        let mut current_list = 0;

        for tasklist in &self.tasklists {
            saved_lists.push_str(tasklist);
            if current_list != last_list {
                saved_lists.push_str("|");
            }
            current_list += 1;
        }

        saved_lists.push_str("\n");

        saved_lists
    }
}
