use std::{path::Path, fs::File, io::BufReader, io::BufRead};

const FILENAME: &str = ".tasks";

use crate::task::{Task, TaskStatus};

// Function to create a .tasks file to store tasks in csv format
pub fn init_list() {
    // Checks to see if a .tasks file exists already
    if !Path::new(FILENAME).exists() {
        // If it doesn't create a .tasks file
        match File::create("./.tasks") {
           Ok(fc) => {
               println!("Tasklist created succesfully!");
               drop(fc);
           }
           // Inform the user if there is a problem with creating the file
           Err(_) => println!("Couldn't create the file!"),
        }
    } else {
        // If the file already exists, inform the user
        println!("A tasks list already exists!");   
    }
}

// Function to read the task file into a Vec of tasks
pub fn read_task_list() -> Vec<Task> {
    // Open the file where tasks are stored
    let file = File::open(FILENAME).expect("Could not open file!");

    // Declare a reader for the file
    let buf_reader = BufReader::new(file);
    
    // Don't quite understand this line fully
    let lines: Vec<String> = buf_reader.lines() // This makes sense, 
        .map(|l| l.expect("Could not parse line")) // ...now this is a bit of a mystery
        .collect(); // This also makes sense, its collecting everything into a vec

    // Go through every line and add the task to a tasklist that the method return
    let mut task_list: Vec<Task> = Vec::new();

    for line in lines {
        let task_vec = read_csv_line(line);
        
        // Figuring out the task's status
        if task_vec.len() == 2 {
            let task_status = match task_vec[1].as_str() {
                "Completed"  => TaskStatus::Completed,
                "InProgress" => TaskStatus::InProgress,
                "NotStarted" => TaskStatus::NotStarted,
                &_ => TaskStatus::NotStarted,
            };

            let task_description = &task_vec[0];

            task_list.push(
                Task { desc: task_description.to_string(), status: task_status }
            );
        } else {
            println!("ERROR WITH LINE");
        }
    }

    task_list
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
