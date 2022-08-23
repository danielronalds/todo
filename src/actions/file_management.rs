use std::{path::Path, fs, fs::File, io::BufReader, io::BufRead};

use crate::task::{Task, TaskStatus};

const FILENAME: &str = ".tasks";


// Function to create a .tasks file to store tasks in csv format
pub fn init_list() -> Result<(), &'static str>{
    // Checks to see if a .tasks file exists already
    if Path::new(FILENAME).exists() {
        return Err("A tasks file already exists!");
    }

    // Creates the file
    let file = File::create(FILENAME);

    // Checks to see if the file was created, returning an error result if it wasn't
    match file {
        Ok(_) => {
            println!("Task list created successfully!");
            Ok(())
        },
        Err(_) => Err("Error creating file!"),
    }
}


// Function to write to the .tasks file to store tasks in csv 
pub fn save_task_list(tasks: Vec<Task>) -> Result<(), &'static str>{
    // Creating a vec to store the data in csv format that will be written to the .tasks file
    let mut tasks_to_write: String = String::new();
    
    // Looping through every task and converting it to csv format, and adding it to the vec
    for task in tasks {
        let line = format!("{},{}\n", task.desc, task.status_to_string());

        tasks_to_write.push_str(&line);
    }
    
    // Writing the string containing all the csv data to the .tasks file 
    let file = fs::write(FILENAME, tasks_to_write);

    // Returns an Err() if the file was unable to be written
    match file {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write to file!") 
    }
}


// Function to read the task file into a Vec of tasks
pub fn read_task_list() -> Result<Vec<Task>, &'static str> {
    // Opening the tasks file, and check if the file was opened successfully, returning an Err() 
    // if it wasn't, so that the run function can handle it
    let file = File::open(FILENAME);

    match &file {
        Ok(file) => file,
        Err(_) => return Err("Couldn't open Tasks file!")
    };

    // Declare a reader for the file
    let buf_reader = BufReader::new(file.unwrap());
    
    // Collecting all the lines into a String Vec
    let lines: Vec<String> = buf_reader.lines().map(|l| {
        l.unwrap_or_else(|err| {
            eprintln!("Could not unwrap line! {}", err);
            String::new()
        })
    }).collect(); 

    // Go through every line and add the task to a tasklist that the method returns
    let mut task_list: Vec<Task> = Vec::new();

    // int to keep track of the line number for reporting errors within the tasks file
    let mut line_num = 1;

    for line in lines {
        let task_vec = read_csv_line(line);
        
        if task_vec.len() != 2 {
            eprintln!("Wrong number of elements in line {line_num}!");
            continue
        }

        // If the line has the correct number of csv elements, then build a Task and push it to
        // the task_list vec, printing an error if the task fails to build
        let new_task = Task::build(&task_vec[0], match task_vec[1].as_str() {
            "Completed"  => TaskStatus::Completed,
            "InProgress" => TaskStatus::InProgress,
            "NotStarted" => TaskStatus::NotStarted,
            &_ => TaskStatus::NotStarted,
        });

        match &new_task {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error on line {line_num}: {}", err);
                continue;
            }
        }

        task_list.push(new_task.unwrap());

        line_num += 1;
    }

    Ok(task_list)
}


// Function to read a string in csv formating
fn read_csv_line(line: String) -> Vec<String>{
    let data_points: Vec<&str> = line.split(",").collect();

    let mut data:Vec<String> = Vec::new();

    for data_point in data_points {
        data.push(data_point.to_string());
    }

    data
}
