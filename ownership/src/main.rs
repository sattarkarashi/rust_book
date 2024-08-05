fn main() {
    // Ownership controls demonstrates the interaction of the code with memory and how it works

    // stack and heap
    // some data types like integers with fixed size dt are saved in stack.

    let x = 5;
    let x_ptr = &x;
    println!("{:p}",x_ptr);
    let y = x;
    let y_ptr = &y;
    println!("x is {x} and y is {y}");
    let y = 8;

    println!("{:p}",y_ptr);
    println!("x is {x} and y is {y}");

    // The data types which don't have fixed sizes, get stored in heap

    let c = String::from("Sato");
    let c_ptr = &c;
    println!("{:p}",c_ptr);
    println!("{c}");

    let d = c;
    let d_ptr = &d;
    println!("{d}");
    println!("{:p}",d_ptr);
    // println!("{c}") will panic because in heap it actually moves the variables this way, while in stack it didn't happen.

    // rust doesn't have GC, and the way it handles the memory is by scope. Whenever the scope is over, it will remove the space with those scopes.

    // a way to deep copy heap values
    let e = d.clone();
    let k = e.clone();
    let j = &e;
    println!("{e} {k}");
    println!("{:p}",j);

    // ownership and functions

    fn get_string(string:String){
        println!("{string} inside the function");
    }

    let str_value = String::from("Sato is here");
    println!("{str_value}");

    get_string(str_value);

    // println!("{str_value}"); You can see that this line will panic, because functions also move the heap values.

    //sometimes we want to pass the value to function but don't let it take its ownership, so we can use reference at these moments

    fn return_string_len(target_string:&String) -> usize {
        let length = target_string.len();
        length
    }

    let my_string = String::from("This is my string");
    println!("{my_string}");

    let my_string_length = return_string_len(&my_string);
    println!("{my_string} and the length is {my_string_length}");

    //as you could see it didn't move the value, it only used its reference inside the function. You can't change the value of a variable when you only
    // have the reference. But if we create a mutable value, we can change it using the reference like below:

    let mut my_mut_string = String::from("This is mutable reference");
    println!("Before changing: {my_mut_string}");
    fn change_mut_variable(target_string:&mut String){
        target_string.push_str(" and I changed it using mutable referencing");
    }

    change_mut_variable(&mut my_mut_string);
    println!("After changing: {my_mut_string}");

    change_mut_variable(&mut my_mut_string);
    println!("After changing: {my_mut_string}");

    // borrowing reference has a restriction, you can't use it to have more than value as mutable reference
    // in the below scenario rust panics because we are using two variable for a mutable reference;
    // let first_mut = &mut my_mut_string;
    // let second_mut = &mut my_mut_string;
    
    // println!("{first_mut}");
    // println!("{second_mut}");

    // but in this following example it doesn't panic because a reference's scope starts where it defined and ends where it is last used.

    let first_mut = &mut my_mut_string;
    println!("{first_mut}");
    // here the scope of the first_mut ends

    let second_mut = &mut my_mut_string;
    println!("{second_mut}");



}
