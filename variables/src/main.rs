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

    // types are immutable in mutable variables, so you can change a mutable variable's type

    // scalar types
    
    let int_scale:i8 = 56;
    println!("This is an i8 signed integer: {int_scale}");
    // integer overflow: based on the types integers are capable of 2**n -1 value and if we put a bigger number,
    // it will start from the beginning and will not process our target number. In release mode, rust doesn't check for integer overflow but
    // in build mode, it checks and panics instead of giving the number.
    //let int_scale:u8 = 256; results in panic

    // numeric opersations 
    let a = 6 + 13;
    let b = 13/2;
    let c = 13%8;
    let d = 18.2*4.5;
    let e = 13.2/3.0;

    println!("The numeric operation results are {a} {b} {c} {d} {e}");

    // compound types

    // tuples are fixed length with different types of values

    let tup: (i32,&str,f64) = (2,"sato",45.5);
    let first_element = tup.0;
    println!("The tuple values are {first_element}");

    // arrays are fixed length with the same type variables.
    let first_arr = [8;5];
    let first_element = first_arr[0];
    println!("The first element of the array is {first_element}");

}
