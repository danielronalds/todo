use std::{path::Path, fs::File};

// Function to create a .tasks file to store tasks in csv format
pub fn init_list() {
    // Checks to see if a .tasks file exists already
    if !Path::new("./.tasks").exists() {
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
