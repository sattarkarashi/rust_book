use std::fs::{self,File};
use std::io::ErrorKind;
use std::io::{self, Read};

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

    // unwrap expect

    // let greeting_file3 = File::open("hello.txt").expect("hello.txt should be included in the this project.");

    // Propagating error: create a function which reads a username from a file

    fn read_username_from_file() -> Result <String, io::Error> {
        let username_file_result = File::open("hello.txt");
        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e)
        };
        let mut username = String::new();

        match username_file.read_to_string(&mut username){
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // The ? operator for propagating errors
    fn read_username_from_file_with_operator() -> Result <String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    // Let's make it even shorter using fs::read_to_string
    fn read_username_from_file_with_fs() -> Result <String, io::Error> {

        fs::read_to_string("hello.txt")
    }

    // We can only use ? operator on functions which return Result <T> and Option <T>

    // Create a function which returns the last character of a line

    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()

    }

    let first_text = "Sato is here";
    let second_text = "";

    let last_char1 = last_char_of_first_line(&first_text);
    let last_char2 = last_char_of_first_line(&second_text);

    println!("{:?} and {:?}",last_char1, last_char2);y 

    // We can have the main function return any types of Errors by adding the return value of Box <dyn Error> and therefore we can use the ? operator in main in this case.

    // We can use custom types instead of repetitive conditional checks for error handling. Remember the guessing game where wanted an input between 1 to 100

    //**** Using conditionals:

    // loop {
    //     let guess: i32 = match guess.trim().parse(){
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     if guess < 1 || guess > 100 {
    //         println!("The secret number should be between 1 and 100");
    //         continue;
    //     }

    //     match guess.cmp(&secret_numer){
    //         ...
    //     }
    // }

    // *** Defining a custom type instead of repeating this logics

    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value > 100 || value < 1 {
                panic!("Guess value must between 1 and 100, but got {}", value);
            }
            Guess {value}
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }




    

}
