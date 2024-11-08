fn main() {
    // Match expressions

    // match value {
    //     pattern1 => expression1,
    //     pattern2 => expression2,
    //     pattern3 => expression3,

    // }


    let some_value: Option<String> = Some(String::from("Sato"));

    match some_value {
        Some(v) => println!("The value is {v}"),
        None => println!("No value"),
    }

    // If let conditionals
    let favorite_color:Option<&str> = None;
    let is_tuesday = false;

    let age:Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite, {color} as background.");
    }else if is_tuesday {
        println!("Tuesday is a green day!");
    }else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        }else {
            println!("Using orange as the background color");

        }

    }else {
        println!("Using blue as the background color");

    }

    // While let conditionals
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}")
    }

    // for loops
    let v = vec!["a", "b", "c"];

    for (index, value) in v.iter().enumerate(){
        println!("{value} is at index {index}");

    }

    // Function parameters
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Coordinates: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

}
