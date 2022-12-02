use crate::task::Task;

/// Const for storing the file name to write to
const FILE_NAME: &str = ".tasks.testing";


/// Enum for storing possible serialization errros
pub enum SerializationErrors {
    FailedToCreateWriter,
    FailedToSerialize,
    FailedToFlush,
}

/// Seralializes the tasks from the given Vec<Task> to the .tasks file
///
/// Parameters
/// tasks:   The tasks list to write to the file
pub fn serialize_tasks(tasks: Vec<Task>) -> Result<(), SerializationErrors> {
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


/// Enum for storing possible deserialization errros
pub enum DeserializationErrors {
    FailedToCreateReader,
    FailedToDeserializeTask
}

/// Deserializes the serializes data in the .tasks file to a Vec<Task>
pub fn deserialize_tasks() -> Result<Vec<Task>, DeserializationErrors> {
    let mut tasks: Vec<Task> = Vec::new();

    let mut reader = match csv::Reader::from_path(FILE_NAME) {
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
