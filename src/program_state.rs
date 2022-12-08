use crate::task::Task;

use crate::config::Config;

use std::fs::{self, File};

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

/// Seralializes the config from the given Config to the config file in a yaml format
///
/// Parameters
/// config:   The config to serialize
pub fn serialize_config(config: Config) -> Result<(), SerializationErrors> {
    let yaml = match serde_yaml::to_string(&config) {
        Ok(yaml) => yaml,
        Err(_) => return Err(SerializationErrors::FailedToSerialize),
    };

    match fs::write(CONFIG_FILE_NAME, yaml) {
        Ok(_) => Ok(()),
        Err(_) => Err(SerializationErrors::FailedToCreateWriter),
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

/// Deserializes the yaml data in the config file to a Config
pub fn deserialize_config() -> Result<Config, DeserializationErrors> {
    let file = match File::open(CONFIG_FILE_NAME) {
        Ok(file) => file,
        Err(_) => return Err(DeserializationErrors::FailedToCreateReader),
    };

    let config: Config = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) =>  return Err(DeserializationErrors::FailedToDeserializeTask)
    };

    Ok(config)
} 
