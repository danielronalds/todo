use serde::{Deserialize, Serialize};

use colored::Colorize;

use std::fmt::Write;

/// Const for representing the default value for num_of_tasks
const DEFAULT_NUM_OF_TASKS: usize = 4;

/// Enum of all possible errors concerning the Config type
#[derive(Debug, PartialEq, Eq)]
pub enum ListErrors {
    ListCannotBeDeleted,
    ListDoesntExist,
    ListAlreadyExists,
}

/// Struct for storing a users config options
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    always_show_list_names: bool,
    always_show_task_ids: bool,
    smart_task_ids: bool,
    num_of_tasks: usize,
    smart_list_names: bool,
    current_list: String,
    lists: Vec<String>,
}

impl Config {
    /// Creates a new Config with a default list of Main
    pub fn new() -> Config {
        Config {
            always_show_list_names: false,
            always_show_task_ids: false,
            smart_task_ids: true,
            num_of_tasks: DEFAULT_NUM_OF_TASKS,
            smart_list_names: true,
            current_list: String::from("Main"),
            lists: vec![String::from("Main")],
        }
    }

    // Returns a cloned iterator of the lists vec
    pub fn lists_iter(&self) -> std::slice::Iter<String> {
        self.lists.iter().clone()
    }

    /// Gets the length of the lists vec
    pub fn lists_len(&self) -> usize {
        self.lists.len()
    }

    /// Gets a clone of the current_list
    pub fn current_list(&self) -> String {
        self.current_list.clone()
    }

    /// Gets the value of always_show_task_ids
    pub fn always_show_task_ids(&self) -> bool {
        self.always_show_task_ids
    }

    /// Sets the value of always_show_task_ids
    pub fn set_always_show_task_ids(&mut self, value: bool) {
        self.always_show_task_ids = value;
    }

    /// Gets the value of always_show_list_names
    pub fn always_show_list_names(&self) -> bool {
        self.always_show_list_names
    }

    /// Sets the value of always_show_list_names
    pub fn set_always_show_list_names(&mut self, value: bool) {
        self.always_show_list_names = value;
    }

    /// Gets the value of smart_list_names
    pub fn smart_list_names(&self) -> bool {
        self.smart_list_names
    }

    /// Sets the value of smart_list_names
    pub fn set_smart_list_names(&mut self, value: bool) {
        self.smart_list_names = value;
    }

    /// Gets the value of smart_task_ids
    pub fn smart_task_ids(&self) -> bool {
        self.smart_task_ids
    }

    /// Sets the value of smart_task_ids
    pub fn set_smart_task_ids(&mut self, value: bool) {
        self.smart_task_ids = value;
    }

    /// Gets the value of num_of_tasks
    pub fn num_of_tasks(&self) -> usize {
        self.num_of_tasks
    }

    /// Sets the value of num_of_tasks
    pub fn set_num_of_tasks(&mut self, value: usize) {
        self.num_of_tasks = value;
    }

    /// Sets the current list
    ///
    /// Parameters:
    /// list:   The list to set the current_list to
    pub fn set_current_list(&mut self, list: String) -> Result<(), ListErrors> {
        // If is_valid_list returns false then the list doesn't exist
        if !(self.is_valid_list(&list)) {
            return Err(ListErrors::ListDoesntExist);
        }

        self.current_list = list;

        Ok(())
    }

    /// Checks if the given list is valid
    ///
    /// Parameters:
    /// list:   The list to check the validity of
    pub fn is_valid_list(&self, list: &String) -> bool {
        self.lists.contains(list)
    }

    /// Adds a list to the config
    ///
    /// Parameters:
    /// list:   The list to add
    pub fn add_list(&mut self, list: String) -> Result<(), ListErrors> {
        // If is_valid_list returns true then the list already exist
        if self.is_valid_list(&list) {
            return Err(ListErrors::ListAlreadyExists);
        }

        self.lists.push(list);

        Ok(())
    }

    /// Removes the given list if it is valid. If the list being deleted is the current_list then
    /// the current_list will be set to the first list
    ///
    /// Parameters
    /// list:   The list to delete
    pub fn delete_list(&mut self, list: String) -> Result<(), ListErrors> {
        if !self.is_valid_list(&list) {
            return Err(ListErrors::ListDoesntExist);
        }

        if self.lists.len() == 1 {
            return Err(ListErrors::ListCannotBeDeleted);
        }

        let index = self.lists.iter().position(|l| l == &list).unwrap();

        self.lists.remove(index);

        if list == self.current_list {
            self.current_list = self.lists[0].clone();
        }

        Ok(())
    }

    /// Returns a string listing all of the lists in the config
    pub fn lists_to_string(&self) -> String {
        let mut lists_string = String::new();

        for list in &self.lists {
            let mut formated_string = format!("{}\n", list.clone());

            if list == &self.current_list() {
                formated_string = format!("{} {}\n", list.clone(), "✔".bright_green());
            }

            lists_string.push_str(&formated_string);
        }

        lists_string
    }

    pub fn config_options_to_string(&self) -> String {
        let mut options_string = String::new();

        writeln!(
            options_string,
            "always_show_task_ids     {}",
            self.always_show_task_ids()
        )
        .unwrap();

        writeln!(
            options_string,
            "smart_task_ids           {}",
            self.smart_task_ids()
        )
        .unwrap();

        writeln!(
            options_string,
            "num_of_tasks             {}",
            self.num_of_tasks()
        )
        .unwrap();

        writeln!(
            options_string,
            "always_show_list_names   {}",
            self.always_show_list_names()
        )
        .unwrap();

        writeln!(
            options_string,
            "smart_list_names         {}",
            self.smart_list_names()
        )
        .unwrap();

        options_string
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests if the constructor works as expected
    fn constructor_works() {
        let new_config = Config::new();

        assert_eq!(
            new_config,
            Config {
                always_show_list_names: false,
                always_show_task_ids: false,
                smart_task_ids: true,
                num_of_tasks: DEFAULT_NUM_OF_TASKS,
                smart_list_names: true,
                current_list: String::from("Main"),
                lists: vec![String::from("Main")]
            }
        )
    }

    #[test]
    /// Tests if always_show_task_ids works
    fn always_show_task_ids_works() {
        let config = Config::new();

        assert!(!config.always_show_task_ids())
    }

    #[test]
    /// Tests if set_always_show_task_ids works
    fn set_always_show_task_ids_works() {
        let mut config = Config::new();

        config.set_always_show_task_ids(true);

        assert!(config.always_show_task_ids())
    }

    #[test]
    /// Tests if always_show_list_names works
    fn always_show_list_names_works() {
        let config = Config::new();

        assert!(!config.always_show_list_names())
    }

    #[test]
    /// Tests if set_always_show_list_names works
    fn set_always_show_list_names_works() {
        let mut config = Config::new();

        config.set_always_show_list_names(true);

        assert!(config.always_show_list_names())
    }

    #[test]
    /// Tests if smart_list_names works
    fn smart_list_names_works() {
        let config = Config::new();

        assert!(config.smart_list_names())
    }

    #[test]
    /// Tests if set_smart_list_names works
    fn set_smart_list_names_works() {
        let mut config = Config::new();

        config.set_smart_list_names(false);

        assert!(!config.always_show_list_names())
    }

    #[test]
    /// Tests if smart_task_ids works
    fn smart_task_ids_works() {
        let config = Config::new();

        assert!(config.smart_task_ids())
    }

    #[test]
    /// Tests if set_smart_task_ids works
    fn set_smart_task_ids_works() {
        let mut config = Config::new();

        config.set_smart_task_ids(false);

        assert!(!config.always_show_list_names())
    }

    #[test]
    /// Tests if num_of_tasks works
    fn num_of_tasks_works() {
        let config = Config::new();

        assert_eq!(config.num_of_tasks(), DEFAULT_NUM_OF_TASKS);
    }

    #[test]
    /// Tests if set_num_of_tasks works
    fn set_num_of_tasks_works() {
        let mut config = Config::new();

        config.set_num_of_tasks(3);

        assert_eq!(config.num_of_tasks(), 3);
    }

    #[test]
    /// Tests current_list works as expected
    fn current_list_works() {
        let config = Config::new();

        assert_eq!(config.current_list(), String::from("Main"))
    }

    #[test]
    /// Tests if is_valid_list works as expected
    fn is_valid_list_works() {
        let config = Config::new();

        assert_eq!(config.is_valid_list(&String::from("Main")), true);
        assert_eq!(config.is_valid_list(&String::from("Not Valid")), false);
    }

    #[test]
    /// Tests if set_current_list works as expected
    fn set_current_list_works() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        config.add_list(listname.clone()).unwrap();

        config.set_current_list(listname.clone()).unwrap();

        assert_eq!(config.current_list(), listname)
    }

    #[test]
    /// Tests if set_current_list fails if the list doesn't exist
    fn set_current_list_fails_if_list_doesnt_exist() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        let error = config.set_current_list(listname.clone()).unwrap_err();

        assert_eq!(error, ListErrors::ListDoesntExist)
    }

    #[test]
    /// Tests if add_list works as expected
    fn add_list_works() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        config.add_list(listname.clone()).unwrap();

        assert_eq!(config.lists, vec![String::from("Main"), listname])
    }

    #[test]
    /// Tests if add_list fails if the list already exists
    fn add_list_fails_if_list_already_exists() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        config.add_list(listname.clone()).unwrap();

        let error = config.add_list(listname.clone()).unwrap_err();

        assert_eq!(error, ListErrors::ListAlreadyExists)
    }

    #[test]
    /// Checks if delete_list works
    fn delete_list_works() {
        let mut config = Config::new();

        let listname = String::from("Backend");

        config.add_list(listname.clone()).unwrap();

        config.delete_list(listname.clone()).unwrap();

        assert_eq!(config.lists, vec![String::from("Main")])
    }

    #[test]
    /// Checks that if the list being deleted is the current_list then the current_list will be set
    /// to the first list
    fn delete_list_changes_current_list_if_current_list_deleted() {
        let mut config = Config::new();

        config.add_list(String::from("Backend")).unwrap();

        config.delete_list(config.current_list()).unwrap();

        assert_eq!(config.current_list(), String::from("Backend"))
    }

    #[test]
    /// Checks if delete_list returns the expected error if the list doesn't exist
    fn delete_list_fails_if_the_list_doesnt_exist() {
        let mut config = Config::new();

        let error = config.delete_list(String::from("Backend")).unwrap_err();

        assert_eq!(error, ListErrors::ListDoesntExist)
    }

    #[test]
    /// Checks if delete_list returns the expected error if the user attempts to delete the last
    /// list
    fn delete_list_fails_if_there_is_only_one_list() {
        let mut config = Config::new();

        let error = config.delete_list(String::from("Main")).unwrap_err();

        assert_eq!(error, ListErrors::ListCannotBeDeleted)
    }
}
