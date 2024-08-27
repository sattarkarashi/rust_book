use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Error handling: we have two types of errors, recoverable and unrecoverable

    // handling errors using panic!
    // panic!("Crash and stop the program"); panic! is a way to handle errors
    
    // By setting the environment variable "Backtrace" to 1 we can see where exactly the code crashed in.

    // ** RUST_BACKTRACE=1 cargo run


    // Recoverable errors using Result: Some errors don't need to stop the program and crash like not finding a file in a location. So, you can by creating the file, remove the error.

    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                }
            }other_error => {
                panic!("Problem opening the file: {:?}", other_error);

            }
        }
    };

    // An alternative to using match with Result<T,E> as a concise and cleaner way 
    let greeting_file_result2 = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    

}
