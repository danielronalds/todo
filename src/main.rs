use std::{env, process};

use todo::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });
    
    todo::run(config);
}
