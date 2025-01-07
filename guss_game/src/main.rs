use std::io;
use rand::Rng;


fn main() {
    println!("Gussing game started!");

    let secret_numer = rand::thread_rng().gen_range(1..=10);
    println!("The random number is {}", secret_numer);

    loop {
        println!("Please input your guss.");

        
    }

}
