use crate::task::Task;
use crate::task::TaskStatus;


/// Function to update a tasks status to inprogress
pub fn start_task(t: &mut Task) {
    match t.status {
        TaskStatus::NotStarted => {
            t.status = TaskStatus::InProgress;
            println!("Started task '{}'", t.desc);
        },
        TaskStatus::InProgress => println!("Task already in progress!"),
        TaskStatus::Completed => println!("Task already completed"),
    }
}


// Function to updatea taks status to completed
pub fn finish_task(t: &mut Task) {
    match t.status {
        TaskStatus::NotStarted => {
            t.status = TaskStatus::Completed;
            println!("Task completed!");
        },
        TaskStatus::InProgress => {
            t.status = TaskStatus::Completed;
            println!("Task completed!");
        },
        TaskStatus::Completed => println!("Task already completed"),
    }
}


// Adds a task to a tasks vec
pub fn add_task(tasks: &mut Vec<Task>, desc: String) {
    let new_task = Task {
        desc: String::from(desc),
        status: TaskStatus::NotStarted,
    };

    tasks.push(new_task);
}


// Removes a task from a tasks vec
pub fn remove_task(tasks: &mut Vec<Task>, index: usize) {
    // Checks to make sure the task is in range to prevent panic!
    if tasks.len() < index {
        let task_desc = &tasks[index].desc;
        println!("Task '{}' removed!", task_desc);

        // if the task does exist, remove it 
        tasks.remove(index);
    }
    else {
        println!("Task does not exist!");
    }
}


// Function to list task
pub fn list_tasks(tasks: &[Task]) {
    // Creating vectors to store the sorted tasks into
    let mut completed_tasks: Vec<&Task> = Vec::new();
    let mut inprogress_tasks: Vec<&Task> = Vec::new();
    let mut notstarted_tasks: Vec<&Task> = Vec::new();

    // Sorting the tasks
    for task in tasks {
        match task.status {
            TaskStatus::Completed => completed_tasks.push(task),
            TaskStatus::InProgress => inprogress_tasks.push(task),
            TaskStatus::NotStarted => notstarted_tasks.push(task),
        } 
    }

    // printing tasks
    println!("Completed tasks:");
    for task in completed_tasks {
        println!("{}", task.desc);
    }

    println!("\nTasks in progress:");
    for task in inprogress_tasks {
        println!("{}", task.desc);
    }
    
    println!("\nTasks not started:");
    for task in notstarted_tasks {
        println!("{}", task.desc);
    }
}


