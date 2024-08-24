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

    // now s3 has been moved and no longer userd.
}
