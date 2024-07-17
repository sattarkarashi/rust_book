use std::io;

fn main() {
    println!("Hello, this is a guessing game");
    println!("Enter your desired guess, please");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("Here is your guess: {guess}");

}
