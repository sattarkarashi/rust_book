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

    // Refutability
    if let Some(x) = Some(5) {
        println!("x is {x}");
    };

    // The following code will warn because x is not refutable and it is not neeeded.
    if let x = 5 {
        println!("x is {x}");
    }

    // Matching literals

    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    // Matching named variables

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}",x);

    // Multiple patterns
    let x = 1;
    match x { 
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Ranges in char values

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }   

    let p = Point { x: 0, y: 7 };   

    let Point { x: a, y: b } = p;
    println!("a: {a}, b: {b}");

    // Matching axis
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),  
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }


    // Destructuring enums

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => println!("Text message: {text}"),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {r}, green {g}, and blue {b}"),
    }

    // Destructuring nested tuples
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {r}, green {g}, and blue {b}");        
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to hue {h}, saturation {s}, and value {v}");     
        }
        _ => (),
    }
}
