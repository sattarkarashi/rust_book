// We move a part of the functions outside main into lib to be easily testable and well structured.
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// fn parse_config(args: &[String]) -> Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config {query, file_path}

// }

// We use a method for config instead of the above commented function which makes it more cleaner and structured.

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        
        if args.len() < 3 {
            return Err("Not enough arguments to extract.")
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})

    }
}
// The separation concern for binary projects asserts separating functions and error handlings
pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

// we go ahead based on the TDD, we write a failing test and then write the code to pass the test, and then refactor the code and then repeat the process

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
        safe, fast, productive.
        Go ahead.";

        assert_eq!(vec!["safe, fast, productive"],search(query,contents));

    }
}

pub fn search <'a> (
    query: &str,
    contents:&'a str,
) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}