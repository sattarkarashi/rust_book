use std::fs::File;

fn main() {
    // Error handling: we have two types of errors, recoverable and unrecoverable

    // handling errors using panic!
    // panic!("Crash and stop the program"); panic! is a way to handle errors
    
    // By setting the environment variable "Backtrace" to 1 we can see where exactly the code crashed in.

    // ** RUST_BACKTRACE=1 cargo run


    // Recoverable errors using Result: Some errors don't need to stop the program and crash like not finding a file in a location. So, you can by creating the file, remove the error.

    let getting_file_result = File::open("hello.txt");

}
