use std::env;
use std::process;

use minigrep::Config;

// Main function should just have logic related to reading config and starting the program
fn main() {
    // Read command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Save command line arguments
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Start the program logic
    // If an error is returning print it and exit with non 0 code
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
