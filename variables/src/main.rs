fn main() {
    // Implicit variable definition
    let imp_var = 6;
    println!("The implicit variable is {imp_var}");

    // Explicit variable definition
    // Also used exp_var again and it is called shadowing
    let exp_var: i32 = 8;
    println!("The explicit variable is {exp_var}");

    let my_name = "Sato";
    println!("The explicit variable is {my_name}");

    //variables by default are immutable in Rust but you can make them mutable this way:
    // mutable vars
    let mut my_age = 31;
    println!("My age is is {my_age}");
    let mut my_age = 32;
    println!("TMy age is {my_age}");

}
