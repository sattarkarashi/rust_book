// We move a part of the functions outside main into lib to be easily testable and well structured.
use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// fn parse_config(args: &[String]) -> Config{
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config {query, file_path}

// }

// We use a method for config instead of the above commented function which makes it more cleaner and structured.

impl Config {
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path string."),
        };


        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})

    }
}
// The separation concern for binary projects asserts separating functions and error handlings
pub fn run(config: Config)-> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };


    for line in results {
        println!("{line}");
    }

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

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));

    }

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Go ahead. Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Go ahead. Trust me";

        assert_eq!(vec!["Rust:", "Go ahead. Trust me"], search_case_insensitive(query, contents));
    }
}

pub fn search <'a> (
    query: &str,
    contents:&'a str,
) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()

}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {

    // to_lowercase method actually creates a new string instead of referencing it.
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        // We add & to query because contains gets string slices not strings. The signature of contains is to take string slices.
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }

    results

}