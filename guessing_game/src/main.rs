use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Hello, this is a guessing game");
    println!("Enter your desired guess, please");
    loop {
    
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter the right format for your guess");
                continue;
            }
        };
        let rand_value = rand::thread_rng().gen_range(1..=100);

        match
        guess.cmp(&rand_value){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => println!("Nailed it!"),
        }
        println!("Here is your guess: {guess} and here is the guess value: {rand_value}");
    }
}
