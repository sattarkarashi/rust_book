fn main() {
    let reference_to_nothing = {
        let s = String::from("hello");
        &s
    }; // s goes out of scope here, making reference_to_nothing a dangling reference
    println!("{}", reference_to_nothing); // This would cause a compilation error
    // Example 1: Returning a reference from a function
    let dangling_ref = dangle();

    // Example 2: Reference to data owned by a different scope
    let another_dangling = {
        let v = vec![1, 2, 3];
        v.first()
    };

    fn dangle() -> &String {             // This won't compile
        let s = String::from("hello");
        &s
    }

}
