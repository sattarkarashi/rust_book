use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {err}");
            process::exit(1);
        });

    // A function to get arguments, we should avoid using lots of logic inside main, instead we should use different parts in different function and each function just do one thing
   

    // We use if let for error handling instead of unwrap_or_else because we don't need the returned value.

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };

    

}


