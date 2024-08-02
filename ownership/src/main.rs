fn main() {
    // Ownership controls demonstrates the interaction of the code with memory and how it works

    // stack and heap
    // some data types like integers with fixed size dt are saved in stack.

    let x = 5;
    let y = x;
    println!("x is {x} and y is {y}");

    // The data types which don't have fixed sizes, get stored in heap

    let c = String::from("Sato");
    println!("{c}");

    let d = c;
    println!("{d}");
    // println!("{c}") will panic because in heap it actually moves the variables this way, while in stack it didn't happen.

    // rust doesn't have GC, and the way it handles the memory is by scope. Whenever the scope is over, it will remove the space with those scopes.
}
