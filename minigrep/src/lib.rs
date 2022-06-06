use std::error::Error;
use std::fs;

// Create a struct responsible for holding config info
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // Constructor for Config struct
    // Takes a vector of strings
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // make sure there were enough arguments passed to the program.
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // clone isn't efficient but it's simple for this use case
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// Takes a Config struct
// Returns a Restult<T, E> enum
// So should return either Ok<T> or Error<E>
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read file
    // Uses ? operator to return error if this call fails
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    // Everything succeeded so return Ok with no value
    Ok(())
}

// function that takes in a query slice and a contents slice
// returns a vector of slices
// uses 'a lifetime check to avoid null references (required by compiler)
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// Add module to hold unit tests
#[cfg(test)]
mod tests {
    // import everything outside this module so it can be tested
    use super::*;

    // a test function
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
