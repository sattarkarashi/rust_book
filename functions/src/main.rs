fn main() {
    // simple function
    // You should always define the type of the function parameter and if you don't, rust panics.
    fn print_value(value: i32){
        println!("Your value is: {value}");
    }
    print_value(5);

    // Function that returns a value
    fn return_value(value: i32)->i32 {
        return value + 1;
    }

    // an alternative for the above function

    fn return_value2(value:i32) -> i32 {
        value + 1
    }
    let return_val = return_value(39);
    let return_val2 = return_value2(18);
    println!("The return value is: {return_val}");
    println!("The second return value is: {return_val2}");

    // expressions and statements
    // rust is an expression based language, expressions return a value while statement doesn't.
    let expr_val = {
        let x = 3;
        x + 5
    };
    println!("The value of the expression is {expr_val}");


    // control flow (if else)

    fn even_or_odd(value:i32) -> String {
        if value % 2 ==0 {
            println!("{value} is even");
            "even".to_string()
        } else {
            println!("{value} is odd");
            "odd".to_string()
        }
    }
    even_or_odd(5);
    even_or_odd(8);

    // loops

    fn loop_counter(value:i32) -> i32 {
        let mut counter = 0;

        loop {
            counter = counter + 1; 
            if counter > value {
                println!("Iterated {counter} times");
                break counter;
            }
            
        }
    }
    loop_counter(32);



}
