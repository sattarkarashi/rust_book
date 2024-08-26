use std::collections::HashMap;
fn main() {
    // Hashmaps are key values structures which values are being hashed using a hash function
    
    // Creating a new hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("Sato"),2);
    println!("{:?}",scores);

    // Accessing values in hashmaps: the get will return an Option<T> and the copied will return a value not &value and unwrap will return the value if there is a key or 0 in this case.

    let name = String::from("Sato");
    let name_score = scores.get(&name).copied().unwrap_or(0);
    println!("{name_score}");

    // Iteration over key and values in hashmaps

    for (key, value) in &scores{
        println!("{key}: {value}");
    }

    // Updating hashmaps: one way is to use score and one way we can first check for the key if exists and then updating the value

    let my_val = scores.entry(String::from("Sati")).or_insert(19);
    let my_val = scores.entry(String::from("Sati")).or_insert(17);

    *my_val += 19;
    println!("{my_val}");
    println!("{:?}",scores);

    // Let's write a code which counts the words in a sentence using hashmaps and how we should update the values

    let my_text = "Hello world, world is beautiful and imagination is powerful.";
    let mut my_map = HashMap::new();
    for word in my_text.split_whitespace(){
        let count = my_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}",my_map);



}
