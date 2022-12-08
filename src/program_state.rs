use crate::task::Task;

use crate::config::Config;

/// Const for storing the file name to write the tasks to
const TASK_FILE_NAME: &str = ".todo/tasks.testing";

/// Const for storing the file name to write the config to
const CONFIG_FILE_NAME: &str = ".todo/config.testing";

/// Enum for storing possible serialization errros
pub enum SerializationErrors {
    FailedToCreateWriter,
    FailedToSerialize,
    FailedToFlush,
}

/// Seralializes the tasks from the given Vec<Task> to the task file
///
/// Parameters
/// tasks:   The tasks list to write to the file
pub fn serialize_tasks(tasks: Vec<Task>) -> Result<(), SerializationErrors> {
    let mut writer = match csv::Writer::from_path(TASK_FILE_NAME) {
        Ok(writer) => writer,
        Err(_) => return Err(SerializationErrors::FailedToCreateWriter),
    };

    for task in tasks {
        match writer.serialize(task) {
            Ok(_) => (),
            Err(_) => return Err(SerializationErrors::FailedToSerialize),
        };
    }

    match writer.flush() {
        Err(_) => Err(SerializationErrors::FailedToFlush),
        Ok(_) => Ok(()),
    }
}

/// Seralializes the config from the given Config to the config file
///
/// Parameters
/// config:   The config to serialize
pub fn serialize_config(config: Config) -> Result<(), SerializationErrors> {
    let mut writer = match csv::Writer::from_path(CONFIG_FILE_NAME) {
        Ok(writer) => writer,
        Err(_) => return Err(SerializationErrors::FailedToCreateWriter),
    };

    match writer.serialize(config) {
        Ok(_) => (),
        Err(_) => return Err(SerializationErrors::FailedToSerialize),
    };

    match writer.flush() {
        Err(_) => Err(SerializationErrors::FailedToFlush),
        Ok(_) => Ok(()),
    }
}

/// Enum for storing possible deserialization errros
pub enum DeserializationErrors {
    FailedToCreateReader,
    FailedToDeserializeTask,
}

/// Deserializes the serializes data in the tasks file to a Vec<Task>
pub fn deserialize_tasks() -> Result<Vec<Task>, DeserializationErrors> {
    let mut tasks: Vec<Task> = Vec::new();

    let mut reader = match csv::Reader::from_path(TASK_FILE_NAME) {
        Ok(writer) => writer,
        Err(_) => return Err(DeserializationErrors::FailedToCreateReader),
    };

    for result in reader.deserialize() {
        let task: Task = match result {
            Ok(task) => task,
            Err(_) => return Err(DeserializationErrors::FailedToDeserializeTask),
        };

        tasks.push(task)
    }

    Ok(tasks)
}

/// Deserializes the serializes data in the config file to a Config
pub fn deserialize_config() -> Result<Config, DeserializationErrors> {
    let mut reader = match csv::Reader::from_path(CONFIG_FILE_NAME) {
        Ok(writer) => writer,
        Err(_) => return Err(DeserializationErrors::FailedToCreateReader),
    };

    // Provides a default config if the file doesn't contain one
    let mut config = Config::new();

    for result in reader.deserialize() {
        // Updates the config to whatever the last line of the config is
        config = match result {
            Ok(task) => task,
            Err(_) => return Err(DeserializationErrors::FailedToDeserializeTask),
        };
    }

    Ok(config)
}
