use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    // A function to get arguments, we should avoid using lots of logic inside main, instead we should use different parts in different function and each function just do one thing
    struct Config {
        query: String,
        file_path: String,
    }

    // fn parse_config(args: &[String]) -> Config{
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config {query, file_path}

    // }

    // We use a method for config instead of the above commented function which makes it more cleaner and structured.
    
    impl Config {
        fn build(args: &[String]) -> Result<Config, &'static str> {
            
            if args.len() < 3 {
                return Err("Not enough arguments to extract.")
            }
            
            let query = args[1].clone();
            let file_path = args[2].clone();

            Ok(Config {query, file_path})

        }
    }

    println!("Searching for {}", config.query);
    println!("In file path {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text: \n {contents}");

}
