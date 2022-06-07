use std::env;
use std::process;

use minigrep::Config;

// Main function should just have logic related to reading config and starting the program
fn main() {
    // Create a new config object by passing in an iterator returned by env::args()
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Start the program logic
    // If an error is returning print it and exit with non 0 code
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
