use serde::{Deserialize, Serialize};

use colored::Colorize;

/// Enum of all possible errors concerning the Config type
#[derive(Debug, PartialEq, Eq)]
pub enum ListErrors {
    ListCannotBeDeleted,
    ListDoesntExist,
    ListAlreadyExists
}

/// Struct for storing a users config options
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    current_list: String,
    lists: Vec<String>,
}

impl Config {
    /// Creates a new Config with a default list of Main
    pub fn new() -> Config {
        Config {
            current_list: String::from("Main"),
            lists: vec![String::from("Main")],
        }
    }

    /// Gets a clone of the current_list
    pub fn current_list(&self) -> String {
        self.current_list.clone()
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
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests if the constructor works as expected
    #[test]
    fn constructor_works() {
        let new_config = Config::new();

        assert_eq!(
            new_config,
            Config {
                current_list: String::from("Main"),
                lists: vec![String::from("Main")]
            }
        )
    }

    /// Tests current_list works as expected
    #[test]
    fn current_list_works() {
        let config = Config::new();

        assert_eq!(config.current_list(), String::from("Main"))
    }

    /// Tests if is_valid_list works as expected
    #[test]
    fn is_valid_list_works() {
        let config = Config::new();

        assert_eq!(config.is_valid_list(&String::from("Main")), true);
        assert_eq!(config.is_valid_list(&String::from("Not Valid")), false);
    }

    /// Tests if set_current_list works as expected
    #[test]
    fn set_current_list_works() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        config.add_list(listname.clone()).unwrap();

        config.set_current_list(listname.clone()).unwrap();

        assert_eq!(config.current_list(), listname)
    }

    /// Tests if set_current_list fails if the list doesn't exist
    #[test]
    fn set_current_list_fails_if_list_doesnt_exist() {
        let mut config = Config::new();

        let listname = String::from("Dev");

        let error = config.set_current_list(listname.clone()).unwrap_err();

        assert_eq!(error, ListErrors::ListDoesntExist)
    }

    /// Tests if add_list works as expected
    #[test]
    fn add_list_works() {
        let mut config = Config::new();
        
        let listname = String::from("Dev");

        config.add_list(listname.clone()).unwrap();

        assert_eq!(config.lists, vec![String::from("Main"), listname])
    }

    /// Tests if add_list fails if the list already exists
    #[test]
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
