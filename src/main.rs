use std::{env, process::exit};

use todo::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        todo::print_error(format!("{}", err).as_str());
        exit(1);
    });
    
    todo::run(config);
}
