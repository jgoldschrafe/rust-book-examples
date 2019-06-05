use std::env;
use std::process;

use refactoring_to_improve_modularity_and_error_handling::{self, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = refactoring_to_improve_modularity_and_error_handling::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}