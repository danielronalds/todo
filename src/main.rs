mod task;

fn main() {
    let mut task1 = task::Task {
        id: 1,
        desc: String::from("Just a simple task"),
        status: task::TaskStatus::NotStarted,
    };

    println!("{:?}", task1.status);

    task1.status = task::TaskStatus::Completed;

    println!("{:?}", task1.status);
}
