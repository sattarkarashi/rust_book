use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, this is a guessing game");
    println!("Enter your desired guess, please");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    let guess: u32 = guess.trim().parse().expect("Enter a valid number");
    let rand_value = rand::thread_rng().gen_range(1..=100);

    match
    guess.cmp(&rand_value){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal => println!("Nailed it!"),
    }
    println!("Here is your guess: {guess} and here is the guess value: {rand_value}");

}
