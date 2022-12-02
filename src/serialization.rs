use crate::task::Task;
use serde::Serialize;

/// Const for storing the file name to write to
const FILE_NAME: &str = ".tasks.testing";

/// Enum for storing possible errros
pub enum SerializationErrors {
    FailedToCreateWriter,
    FailedToSerialize,
    FailedToFlush,
}

/// Writes tasks from .tasks file
///
/// Parameters
/// tasks:   The tasks list to write to the file
pub fn writer(tasks: Vec<Task>) -> Result<(), SerializationErrors> {
    let mut writer = match csv::Writer::from_path(FILE_NAME) {
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
