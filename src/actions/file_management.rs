use std::{path::Path, fs, fs::File, io::BufReader, io::BufRead};

use crate::print_error;
use crate::task::{Task, TaskStatus};

use crate::user_config::UserConfig;

const FILENAME: &str = ".tasks.testing";


/// Creates a new .task file in the current directory if one does not already exist
pub fn init_list() -> Result<&'static str, &'static str>{
    // Checks to see if a .tasks file exists already
    if Path::new(FILENAME).exists() {
        return Err("A tasks file already exists!");
    }

    // Creating a default configiration to write to the new list 
    let mut save_data: String = String::new();

    let user_config = UserConfig::default();

    // Saving the user's config
    save_data.push_str(&user_config.to_save_format());
    save_data.push_str(&user_config.tasklists_to_save_format());
    
    // Writing the string containing all the csv data to the .tasks file, and Returns an Err() if 
    // the file was unable to be written
    match fs::write(FILENAME, save_data) {
        Ok(_) => Ok("Task list created successfully!"),
        Err(_) => Err("Failed to write to file!") 
    }
}


/// Deletes the .tasks in the current directory
pub fn delete_list() -> Result<&'static str, &'static str> {
    print_error("This will delete the tasks list in this directory, are you sure? [y/N]\n");

    let mut confirmation = String::new();

    std::io::stdin().read_line(&mut confirmation).expect("Couldn't read line");

    match confirmation.trim() {
        "y" | "Y" | "yes" | "YES"  => {
            let deleted = fs::remove_file(FILENAME);

            match deleted {
                Ok(_) => return Ok("Tasklist deleted!"),
                Err(_) => return Err("Tasklist could not be deleted!")
            };
        },
        _ => return Err("Tasklist was not deleted"),
    }
}


/// Writes the current state of the program to the .tasks file
///
/// Parameters
/// tasks:          The vec containing all the tasks
/// users_config:   The user's settings
pub fn save_task_list(tasks: Vec<Task>, users_config: UserConfig) -> Result<(), &'static str>{
    // Creating a vec to store the data in csv format that will be written to the .tasks file
    let mut save_data: String = String::new();

    // Saving the user's config
    save_data.push_str(&users_config.to_save_format());
    save_data.push_str(&users_config.tasklists_to_save_format());
    
    for task in tasks {
        let line = format!("{}|{}|{}\n", task.desc, task.status_to_string(), task.list);
        save_data.push_str(&line);
    }
    
    // Writing the string containing all the csv data to the .tasks file, and Returns an Err() if 
    // the file was unable to be written
    match fs::write(FILENAME, save_data) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write to file!") 
    }
}


/// Reads the list of tasks and the users config from the .tasks file in the current directory
pub fn read_task_list() -> Result<(Vec<Task>, UserConfig), &'static str> {
    // Opening task file
    let lines: Vec<String> = match read_file(FILENAME) {
        Ok(lines) => lines,
        Err(err) => return Err(err),
    };

    // Go through every line and add the task to a tasklist that the method returns
    let mut tasks: Vec<Task> = Vec::new();

    let mut config = UserConfig::default();

    // int to keep track of the line number for reporting errors within the tasks file
    let mut line_num = 1;

    for line in lines {
        let line_vec = read_csv_line(line);
        
        // Grabs the config out of the first line of the file
        if line_num == 1 {
            config = UserConfig::build(line_vec);
            line_num += 1;
            continue
        }

        // Grabs the exisiting tasklists out of the second line
        if line_num == 2 {
            config.tasklists = line_vec;
            line_num += 1;
            continue
        }

        if line_vec.len() != 3 {
            eprintln!("Wrong number of elements in line {line_num}!");
            continue
        }

        // If the line has the correct number of csv elements, then build a Task and push it to
        // the task_list vec, printing an error if the task fails to build
        let new_task = Task::build(
            line_vec[2].clone(), line_vec[0].clone(), match line_vec[1].as_str() {
                "Completed"  => TaskStatus::Completed,
                "InProgress" => TaskStatus::InProgress,
                "NotStarted" => TaskStatus::NotStarted,
                &_ => TaskStatus::NotStarted,
            });

        tasks.push(match new_task {
            Ok(new_task) => new_task,
            Err(err) => {
                print_error(format!("Error on line {line_num}: {}", err).as_str());
                continue;
            }
        });

        line_num += 1;
    }

    Ok((tasks, config))
}


/// Opens the file with the given file_name and returns its lines as a Vec<String>
///
/// Parameters
/// file_name:  The name of the file to read
fn read_file(file_name: &str) -> Result<Vec<String>, &'static str> {
    // Opening the tasks file, and check if the file was opened successfully, returning an Err() 
    // if it wasn't, so that the run function can handle it
    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open Tasks file, Try running init to create a tasks file!")
    };

    // Declare a reader for the file
    let buf_reader = BufReader::new(file);
    
    // Collecting all the lines into a String Vec
    let lines: Vec<String> = buf_reader.lines().map(|l| {
        l.unwrap_or_else(|err| {
            print_error(format!("Could not unwrap line! {}", err).as_str());
            String::new()
        })
    }).collect();

    Ok(lines)
}


/// Reads the given string, and returns the CSV data from that line 
///
/// Parameters
/// line:  The line containg the CSV data
fn read_csv_line(line: String) -> Vec<String> {
    let data_points: Vec<&str> = line.split("|").collect();

    let mut data:Vec<String> = Vec::new();

    for data_point in data_points {
        data.push(data_point.to_string());
    }

    data
}
