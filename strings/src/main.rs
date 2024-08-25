fn main() {
    // Two ways to define strings

    let first_str = String::from("First definition method.");

    let second_str = "Second Definition Method".to_string();

    println!("First one is {first_str} and the second one is {second_str}");

    // *** String are actually a wrapper around vector indeed. They have some specifications.

    // Updating strings
    let mut s = String::from("This is a string.");
    let s2 = "And this is the pushed value.";
    s.push_str(s2);
    println!("{s} and {s2}");

    // push_str didn't take ownership of s2.

    let s3 = String::from("Imagination is important.");
    let s4 = String::from("Let's imagine.");

    let s5 = s3 + &s4;
    println!("{s5}");

    // now s3 has been moved and no longer used.

    // using format for adding strings. It uses a macro and uses the reference of the values so it doesn't take ownership.
    
    let s6 = format!("{s5}{s4}");
    println!("{s6}");

    // indexing: Strings doesn't support common indexing which was valid in vectors, and that's mainly because of how strings are stored in memory. Some characters can take up to two bytes and 
    // trying to get certain indexes will result in mistakes and it wouldn't return the right index we are expecting. 

    // slicing strings in rust
    let s7 = String::from("привет");
    let slice_s7 = &s7[0..4];
    println!("{slice_s7}");

    
    // Well, we need to be careful, because it only returns the first 4 bytes of the string instead of the first four characters.


    // Iterating over strings: You should decide whether you want to work with bytes or chars

    for _char in s7.chars(){
        println!("{_char}");
    }

    for _byte in s7.bytes(){
        println!("{_byte}");
    }

    // Overall strings are complicated because of their nature and Rust to avoid different errors, has gone through this complexity which is a good tradeoff indeed.
}
