// Public facing modules
pub mod args;
pub mod config;
pub mod task;

// Private modules
mod program_state;
mod task_management;

use crate::args::{
    CleanupCommand, ConfigCommand, DeleteCommand, FinishCommand, ListCommand, RestartCommand,
    StartCommand, TasksCommand, UpdateCommand,
};

use std::fs;

use colored::Colorize;

use crate::task::{Task, TaskErrors, TaskStatus};

use crate::config::{Config, ListErrors};

use crate::task_management::{TaskManagementErrors, UpdateTaskErrors};

use crate::program_state::{DeserializationErrors, SerializationErrors};

/// Reads the tasks file and returns a Vec<Task>
pub fn read_tasks_file() -> Result<Vec<Task>, &'static str> {
    match program_state::deserialize_tasks() {
        Ok(tasks_vec) => Ok(tasks_vec),
        Err(err) => match err {
            DeserializationErrors::FailedToCreateReader => {
                // TODO refactor this
                // Asking the user if they'd like to create a tasks file in the directory
                println!("No task file found, would you like to create one? [Y/n]");

                let mut answer = String::new();

                std::io::stdin().read_line(&mut answer).unwrap();

                let answer = answer.trim().to_lowercase();

                if answer == "n" || answer == "no" {
                    return Err("Task file not created");
                }

                // Here we just ignore any errors created, as if an error is created it is most
                // likely due to the folder already existing. Otherwise if the program has no read
                // or write privilages then the whole program wont work and therefore will fail to
                // serialiaze, producing a valid message there.
                fs::create_dir(".todo").unwrap_or(());

                Ok(Vec::new())
            }
            DeserializationErrors::FailedToDeserializeTask => Err("Couldn't read task!"),
        },
    }
}

/// Write the given Vec<Task> to the tasks file
///
/// Parameters
/// tasks:   The vec of tasks to write to the tasks file
pub fn write_tasks_file(tasks: Vec<Task>) -> Result<(), &'static str> {
    match program_state::serialize_tasks(tasks) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            SerializationErrors::UnableToCreateWriter => Err("Failed to create the writer!"),
            SerializationErrors::FailedToSerialize => Err("Failed to serialize the tasks!"),
            SerializationErrors::CouldntFlush => Err("Could not flush!"),
        },
    }
}

/// Reads the config file and returns a Config regardless of any errors
pub fn read_config_file() -> Config {
    match program_state::deserialize_config() {
        Ok(config) => config,
        // Currently any errors will just return a default Config, this might change later though
        Err(_) => Config::new(),
    }
}

/// Write the given Config to the config file
/// Parameters
/// config:   The Config to write to the config file
pub fn write_config_file(config: Config) -> Result<(), &'static str> {
    match program_state::serialize_config(config) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            SerializationErrors::UnableToCreateWriter => Err("Failed to create the writer!"),
            SerializationErrors::FailedToSerialize => Err("Failed to serialize the config!"),
            // serialize_config only produces the errors above
            _ => Err("Unknown error"),
        },
    }
}

/// Deletes the .todo file and everything in it
pub fn nuke_todo() -> &'static str {
    // Asking the user if they'd like to create a tasks file in the directory
    println!("Are you sure you want to delete todo in this directory? [y/N]");

    let mut answer = String::new();

    std::io::stdin().read_line(&mut answer).unwrap();

    let answer = answer.trim().to_lowercase();

    if answer == "y" || answer == "yes" {
        match fs::remove_dir_all(".todo") {
            Ok(_) => return "The .todo dir has been nuked!",
            Err(_) => return "The .todo dir couldn't be nuked!",
        }
    }

    ""
}

/// Consumes the given vec and returns two vecs of tasks, with the first one containing all the
/// tasks in the current list, and the second containg the rest of the tasks
///
/// Parameters
/// task_vec:   The unfiltered Vec<Task>
/// config:     The user's config
pub fn filter_task_vec(task_vec: Vec<Task>, config: &Config) -> (Vec<Task>, Vec<Task>) {
    let mut tagged_tasks: Vec<Task> = Vec::new();

    let mut other_tasks: Vec<Task> = Vec::new();

    for task in task_vec {
        if task.list() == config.current_list() {
            tagged_tasks.push(task);
            continue;
        }
        other_tasks.push(task);
    }

    (tagged_tasks, other_tasks)
}

/// Lists the tasks in the given vec
///
/// Parameters
/// tasks:       The task vec to list
/// config:      The user's config
/// arguments:   The arguments for the command from the cli
pub fn list_tasks(
    tasks: &mut Vec<Task>,
    other_tasks: &[Task],
    config: &Config,
    arguments: TasksCommand,
) -> Result<String, &'static str> {
    // Seeing if the user wants to sort the current list
    if arguments.sort {
        // Ignoring the error this produces as if the list is empty then the listing of the
        // function will print the same error
        task_management::sort_tasks(tasks).unwrap_or(());
    }

    // Seeing if the user wants to list all lists
    if arguments.all {
        match task_management::list_all_tasks(tasks, other_tasks, config) {
            Ok(list) => return Ok(list),
            Err(err) => match err {
                // This is the only possible error
                TaskManagementErrors::EmptyTasklist => {
                    return Err("There are no tasks in any list!")
                }
                // Covering any other errors for now in case the function changes
                _ => return Err("An unknown error has occured!"),
            },
        }
    }

    match task_management::list_tasks(tasks, config) {
        Ok(list) => Ok(list),
        Err(err) => match err {
            // This is the only possible error
            TaskManagementErrors::EmptyTasklist => Err("There are no tasks in the list!"),
            // Covering any other errors for now in case the function changes
            _ => Err("An unknown error has occured!"),
        },
    }
}

/// Sorts the tasks in the given vec
///
/// Parameters
/// tasks:   The task vec to sort
pub fn sort_list(tasks: &mut Vec<Task>) -> Result<(), &'static str> {
    match task_management::sort_tasks(tasks) {
        Ok(_) => Ok(()),
        Err(err) => match err {
            TaskManagementErrors::EmptyTasklist => Err("There are no tasks in the list!"),
            _ => Err("An unknown error has occured!"),
        },
    }
}

/// Removes any Completed task in the given vec
///
/// Parameters
/// tasks:   The task vec to sort
pub fn cleanup_list(
    tasks: &mut Vec<Task>,
    other_tasks: &mut Vec<Task>,
    arguments: CleanupCommand,
) -> &'static str {
    // Removing completed tasks from the current list
    tasks.retain(|task| task.status() != TaskStatus::Completed);

    // Removing completed tasks from all the lists if the all flag is used
    if arguments.all {
        other_tasks.retain(|task| task.status() != TaskStatus::Completed);

        // Early returning a different message to print
        return "Removed all Completed Tasks!";
    }

    "Removed Completed Tasks from the current list!"
}

/// Creates a new task. This handles any errors and returns an appropriate error message
/// This approach will most likely change however, or this function moved
///
/// Parameters
/// arguments:   The arguments for the command from the cli
/// config:      The user's config
pub fn new_task(arguments: String, config: &Config) -> Result<Task, &'static str> {
    let list = config.current_list();

    let task = match Task::new(arguments, TaskStatus::NotStarted, list) {
        Ok(task) => task,
        Err(err) => match err {
            TaskErrors::EmptyDescription => {
                return Err("Tasks cannot have empty descriptions!");
            }
            TaskErrors::EmptyList => return Err("A task must have a list!"),
        },
    };

    Ok(task)
}

/// Triggers the add_mode, which creates a prompt that allows the user to add task rapidly, by only
/// having to type the tasks description
///
/// Parameters
/// tasks:    The task vec to add the tasks too
/// config:   The user's config
pub fn add_mode(tasks: &mut Vec<Task>, config: &Config) {
    // Opening blurb
    print_info("To add a task type the description and press enter, to exit type x");

    // Beginning the loop
    loop {
        let mut description = String::new();

        // Getting the user input
        std::io::stdin().read_line(&mut description).unwrap();

        // Trimming the output
        description = description.trim().to_string();

        // Checking if the user wants to exit
        if description.to_lowercase() == "x" {
            if config.command_feedback() {
                print_info("Exited!");
            }
            return;
        }

        // Attempting to create the task, and if there is an error printing it and continuing to
        // the next iteration of the loop
        match new_task(description, config) {
            Ok(task) => {
                tasks.push(task);
                if config.command_feedback() {
                    print_info("Added task!");
                }
            }
            Err(err) => {
                print_info(err);
                continue;
            }
        }
    }
}

/// Pretty prints the given message to the console
pub fn print_info(message: &str) {
    let symbol = format!("[{}]", "!".bright_blue()).bold();

    println!("{} {}", symbol, message)
}

/// Updates the description of the task at the given task_id in the given task vec
///
/// Parameters
/// tasks:       The task vec to delete from
/// arguments:   The arguments for the command from the cli
pub fn update_task(tasks: &mut Vec<Task>, arguments: UpdateCommand) -> &'static str {
    // Converting the task_id to an index
    let index = task_id_to_index(arguments.task_id);

    // Getting the description
    let mut description = arguments.new_description;

    // Appending the new description if the append flag was used
    if arguments.append {
        description = tasks[index].description() + &description;
    }

    match task_management::update_task_description(tasks, index, description) {
        Ok(_) => "Task updated successfully!",
        Err(err) => match err {
            UpdateTaskErrors::ManagementErrors(error) => match error {
                TaskManagementErrors::EmptyTasklist => "No tasks found!",
                TaskManagementErrors::TaskDoesntExist => "Task not found!",
                _ => "Unknown error!",
            },
            UpdateTaskErrors::TaskErrors(error) => match error {
                TaskErrors::EmptyDescription => "Tasks cannot have empty descriptions!",
                _ => "Unknown error!",
            },
        },
    }
}

/// Deletes a task/s from the list. This function handles the errors and returns a str containing a
/// message to print
///
/// Parameters
/// tasks:       The task vec to delete from
/// arguments:   The arguments for the command from the cli
pub fn delete_task(tasks: &mut Vec<Task>, arguments: DeleteCommand) -> &'static str {
    // Sorting the indexes and removing duplicates
    let indexs = sort_and_filter_task_ids(arguments.task_ids);

    // Looping through the given indexes in reverse order to remove them to prevent deleting the
    // wrong items or attempting to delete at an index that is no longer in the bounds of the vec
    for i in (0..indexs.len()).rev() {
        // Converting the index to a task id
        let index = task_id_to_index(indexs[i]);

        // Using an if let here as I don't care about the Ok variant
        if task_management::delete_task(tasks, index).is_err() {
            return "Task doesn't exist!";
        }
    }

    // Returning different messages based on whether there were multiple tasks to delete
    if indexs.len() > 1 {
        return "Tasks deleted!";
    }

    "Task deleted!"
}

/// Starts the tasks at the given ids in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn start_task(tasks: &mut Vec<Task>, arguments: StartCommand) -> &'static str {
    // Sorting the indexes and removing duplicates
    let indexes = sort_and_filter_task_ids(arguments.task_ids);

    for index in &indexes {
        let index = task_id_to_index(*index);

        // Using an if let statement as only the Err variant is needed
        if let Err(err) = task_management::update_task_status(tasks, index, TaskStatus::InProgress)
        {
            // Generating the error message
            let error_message = match err {
                TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already in progress!",
                TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
                TaskManagementErrors::EmptyTasklist => "No tasks found!",
            };

            return error_message;
        }
    }

    // Returning a success message with a plural if more than one task was started
    if indexes.len() > 1 {
        return "Tasks have been started!";
    }

    "Tasks has been started!"
}

/// Finishes the task at the given id in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn finish_task(tasks: &mut Vec<Task>, arguments: FinishCommand) -> &'static str {
    // Sorting the indexes and removing duplicates
    let indexes = sort_and_filter_task_ids(arguments.task_ids);

    for index in &indexes {
        let index = task_id_to_index(*index);

        if let Err(err) = task_management::update_task_status(tasks, index, TaskStatus::Completed) {
            let error_message = match err {
                TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already completed",
                TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
                TaskManagementErrors::EmptyTasklist => "No tasks found!",
            };

            return error_message;
        }
    }

    // Returning a success message with a plural if more than one task was completed
    if indexes.len() > 1 {
        return "Tasks have been completed!";
    }

    "Task has been completed!"
}

/// Restarts the task at the given id in the given tasks vec
///
/// Parameters
/// tasks:       The task vec the tasks belongs to
/// arguments:   The arguments for the command from the cli
pub fn restart_task(tasks: &mut Vec<Task>, arguments: RestartCommand) -> &'static str {
    // Sorting the indexes and removing duplicates
    let indexes = sort_and_filter_task_ids(arguments.task_ids);

    for index in &indexes {
        let index = task_id_to_index(*index);

        if let Err(err) = task_management::update_task_status(tasks, index, TaskStatus::NotStarted)
        {
            let error_message = match err {
                TaskManagementErrors::TaskAlreadyGivenStatus => "Task is already Not Started",
                TaskManagementErrors::TaskDoesntExist => "Task doesn't exist",
                TaskManagementErrors::EmptyTasklist => "No tasks found!",
            };

            return error_message;
        }
    }

    // Returning a success message with a plural if more than one task was completed
    if indexes.len() > 1 {
        return "Tasks have been restarted!";
    }

    "Task has been restarted!"
}

/// Converts a task_id to an index, preventing a runtime panic from attempting to subtract with
/// overflow
///
/// Parameters
/// task_id:   The task_id to convert to an index
fn task_id_to_index(task_id: usize) -> usize {
    let mut index = task_id;

    // Take one off of the index if not already zero (to prevent runtime panic) as Task ID's start
    // at 1 not 0
    index = index.saturating_sub(1);

    index
}

/// Manages the list command
///
/// Parameters
/// config:   The config to manage the list from
/// arguments:   The arguments form the cli
pub fn manage_lists(config: &mut Config, arguments: ListCommand) -> Option<String> {
    // Checking if the user wants to create a list
    if let Some(list_name) = arguments.new {
        let return_message;

        match config.add_list(list_name) {
            Ok(_) => return_message = "List addded!".to_owned(),
            Err(err) => match err {
                ListErrors::ListAlreadyExists => {
                    return_message = "That list already exists!".to_owned()
                }
                _ => return_message = "This error cannot occur".to_owned(),
            },
        };

        if !config.command_feedback() {
            return None;
        }

        return Some(return_message);
    }

    // Checking if the user wants to switch to a list
    if let Some(list_name) = arguments.switch {
        let return_message;

        match config.set_current_list(list_name) {
            Ok(_) => return_message = "Switched Lists!".to_owned(),
            Err(err) => match err {
                ListErrors::ListDoesntExist => {
                    return_message = "That list doesn't exist!".to_owned()
                }
                _ => return_message = "This error cannot occur".to_owned(),
            },
        };

        if !config.command_feedback() {
            return None;
        }

        return Some(return_message);
    };

    // Checking if the user wants to delete a list
    if let Some(list_name) = arguments.delete {
        let return_message;

        match config.delete_list(list_name) {
            Ok(_) => return_message = "Deleted List!".to_owned(),
            Err(err) => match err {
                ListErrors::ListDoesntExist => {
                    return_message = "That list doesn't exist!".to_owned()
                }
                ListErrors::ListCannotBeDeleted => {
                    return_message = "You must have at least one list!".to_owned()
                }
                _ => return_message = "This error cannot occur".to_owned(),
            },
        };

        if !config.command_feedback() {
            return None;
        }

        return Some(return_message);
    };

    // Default behaviour is listing the lists
    Some(config.lists_to_string())
}

/// Manages the configure command, which allows the user to change behaviour about the program
///
/// Parameters
/// config:      The user's config
/// arguments:   The arguments form the cli
pub fn manage_config(config: &mut Config, arguments: ConfigCommand) -> String {
    // Checks if the user wants to change always_show_task_ids
    if let Some(value) = arguments.always_show_task_ids {
        config.set_always_show_task_ids(value);
        return format!("Set always_show_task_ids to {}", value);
    }

    // Checks if the user wants to change always_show_list_name
    if let Some(value) = arguments.always_show_list_name {
        config.set_always_show_list_names(value);
        return format!("Set always_show_list_name to {value}");
    }

    // Checks if the user wants to change command_feedback
    if let Some(value) = arguments.command_feedback {
        config.set_command_feedback(value);
        return format!("Set command_feedback to {value}");
    }

    // Checks if the user wants to change smart_list_names
    if let Some(value) = arguments.smart_list_names {
        config.set_smart_list_names(value);
        return format!("Set smart_list_names to {value}");
    }

    // Checks if the user wants to change smart_task_ids
    if let Some(value) = arguments.smart_task_ids {
        config.set_smart_list_names(value);
        return format!("Set smart_task_ids to {value}");
    }

    // Checks if the user wants to change num_of_tasks
    if let Some(value) = arguments.num_of_tasks {
        config.set_num_of_tasks(value);
        return format!("Set num_of_tasks to {value}");
    }

    format!(
        "{}                   {}\n{}",
        "Option".underline(),
        "Value".underline(),
        config.config_options_to_string()
    )
}

/// Sorts and removes duplicates in the given Vec<usize>
fn sort_and_filter_task_ids(mut indexes: Vec<usize>) -> Vec<usize> {
    // Sorting the indexes and removing duplicates
    indexes.sort();
    indexes.dedup();
    indexes
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::args::AddCommand;

    #[test]
    /// Tests if the new_task command works as expected
    fn new_task_works() {
        let config = Config::new();

        let description = String::from("This is a basic task");

        let list = config.current_list();

        let expected_task = Task::new(description.clone(), TaskStatus::NotStarted, list).unwrap();

        let arguments = AddCommand {
            description: Some(description),
        };

        let genereated_task = new_task(arguments.description.unwrap(), &config).unwrap();

        assert_eq!(expected_task, genereated_task)
    }

    #[test]
    /// Tests if the filters_tasks function works as expected
    fn filter_tasks_works() {
        let config = Config::new();

        let tasks_vec = vec![
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Other"),
            )
            .unwrap(),
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Another"),
            )
            .unwrap(),
        ];

        let filtered_vecs = filter_task_vec(tasks_vec, &config);

        assert_eq!(
            filtered_vecs.0,
            vec![
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Main"),
                )
                .unwrap(),
                Task::new(
                    String::from("Another basic task"),
                    TaskStatus::Completed,
                    String::from("Main"),
                )
                .unwrap(),
            ]
        );

        assert_eq!(
            filtered_vecs.1,
            vec![
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Other"),
                )
                .unwrap(),
                Task::new(
                    String::from("A basic task"),
                    TaskStatus::Completed,
                    String::from("Another"),
                )
                .unwrap(),
            ]
        );
    }

    #[test]
    /// Tests if the cleanup_list function works as expected
    fn cleanup_list_works() {
        let arguments = CleanupCommand { all: false };

        let mut tasks = vec![
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task"),
                TaskStatus::InProgress,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let mut other_tasks = vec![Task::new(
            String::from("A basic task"),
            TaskStatus::Completed,
            String::from("Main"),
        )
        .unwrap()];

        cleanup_list(&mut tasks, &mut other_tasks, arguments);

        assert_eq!(
            tasks,
            vec![
                Task::new(
                    String::from("Another basic task"),
                    TaskStatus::InProgress,
                    String::from("Main"),
                )
                .unwrap(),
                Task::new(
                    String::from("Yet another basic task"),
                    TaskStatus::NotStarted,
                    String::from("Main"),
                )
                .unwrap(),
            ]
        )
    }

    #[test]
    /// Tests if cleanup_list works if the all flag is added
    fn cleanup_list_with_all_flag_works() {
        let arguments = CleanupCommand { all: true };

        let mut tasks = vec![
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task"),
                TaskStatus::InProgress,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let mut other_tasks = vec![
            Task::new(
                String::from("A basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Another basic task"),
                TaskStatus::InProgress,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        cleanup_list(&mut tasks, &mut other_tasks, arguments);

        assert_eq!(
            tasks,
            vec![
                Task::new(
                    String::from("Another basic task"),
                    TaskStatus::InProgress,
                    String::from("Main"),
                )
                .unwrap(),
                Task::new(
                    String::from("Yet another basic task"),
                    TaskStatus::NotStarted,
                    String::from("Main"),
                )
                .unwrap(),
            ]
        );

        assert_eq!(
            other_tasks,
            vec![
                Task::new(
                    String::from("Another basic task"),
                    TaskStatus::InProgress,
                    String::from("Main"),
                )
                .unwrap(),
                Task::new(
                    String::from("Yet another basic task"),
                    TaskStatus::NotStarted,
                    String::from("Main"),
                )
                .unwrap(),
            ]
        )
    }

    #[test]
    /// Tests if the delete_task function returns the correct messages
    fn delete_task_returns_correct_message() {
        let tasks = vec![
            Task::new(
                String::from("Another basic task"),
                TaskStatus::InProgress,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        // Declaring multiple task ids
        let arguments = DeleteCommand {
            task_ids: vec![1, 2],
        };

        // Asserting a plural tasks is returned when there are multiple tasks
        assert_eq!(delete_task(&mut tasks.clone(), arguments), "Tasks deleted!");

        // Declaring a singular task id
        let arguments = DeleteCommand { task_ids: vec![1] };

        // Asserting a plural tasks is returned when there are multiple tasks
        assert_eq!(delete_task(&mut tasks.clone(), arguments), "Task deleted!");
    }

    #[test]
    /// Tests if the task_id_to_index function works
    fn task_id_to_index_works() {
        let task_id: usize = 1;

        let index = task_id_to_index(task_id);

        assert_eq!(index, 0)
    }

    #[test]
    /// Tests if the task_id_to_index function can handle task_id being 0 and not create a panic
    /// from attempting to subtract with overflow
    fn task_id_to_index_handles_zero_as_input() {
        let task_id: usize = 0;

        let index = task_id_to_index(task_id);

        assert_eq!(index, 0)
    }

    #[test]
    /// Tests if the start command works with multiple ids
    fn start_task_with_multiple_task_ids_works() {
        let mut tasks_vec = vec![
            Task::new(
                String::from("Another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let arguments = StartCommand {
            task_ids: vec![1, 2],
        };

        start_task(&mut tasks_vec, arguments);

        assert_eq!(tasks_vec[0].status(), TaskStatus::InProgress);
        assert_eq!(tasks_vec[1].status(), TaskStatus::InProgress);
    }

    #[test]
    /// Tests if the start command works with multiple ids
    fn delete_task_with_multiple_task_ids_works() {
        let mut tasks_vec = vec![
            Task::new(
                String::from("Another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let arguments = DeleteCommand {
            task_ids: vec![1, 2],
        };

        delete_task(&mut tasks_vec, arguments);

        assert_eq!(tasks_vec.len(), 0);
    }

    #[test]
    /// Tests if the finish_task function works with multiple task ids
    fn finish_task_with_multiple_task_ids_works() {
        let mut tasks_vec = vec![
            Task::new(
                String::from("Another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::NotStarted,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let arguments = FinishCommand {
            task_ids: vec![1, 2],
        };

        finish_task(&mut tasks_vec, arguments);

        assert_eq!(tasks_vec[0].status(), TaskStatus::Completed);
        assert_eq!(tasks_vec[1].status(), TaskStatus::Completed);
    }

    #[test]
    /// Tests if the restart_task function works with multiple task ids
    fn restart_task_with_multiple_task_ids_works() {
        let mut tasks_vec = vec![
            Task::new(
                String::from("Another basic task"),
                TaskStatus::Completed,
                String::from("Main"),
            )
            .unwrap(),
            Task::new(
                String::from("Yet another basic task"),
                TaskStatus::InProgress,
                String::from("Main"),
            )
            .unwrap(),
        ];

        let arguments = RestartCommand {
            task_ids: vec![1, 2],
        };

        restart_task(&mut tasks_vec, arguments);

        assert_eq!(tasks_vec[0].status(), TaskStatus::NotStarted);
        assert_eq!(tasks_vec[1].status(), TaskStatus::NotStarted);
    }

    #[test]
    fn sort_and_filter_task_ids_works() {
        let indexes = vec![1, 2, 2, 5, 1];

        let indexes = sort_and_filter_task_ids(indexes);

        assert_eq!(indexes, vec![1, 2, 5]);
    }
}
