fn main() {
    // we create a function to calculate the area of a rectangle
    let width = 30;
    let height = 40;
    let simple_area = area_calculator(height,width);

    println!("The area of the rectangle using the simple method is {simple_area} square meters. ");

    // Calculating the area using a tuple, as you can see it gives a little more structure in expense of losing dimension names

    let dimensions = (30, 40);
    let tuple_area = area_calculator_tuple(dimensions);
    println!("The area of the rectangle using the tuple method is {tuple_area} square meters. ");

    // Calculating the area using structs
    let rectangle = Rectangle{
        height:40,
        width:30
    };
    //using #[derive[Debug]] we can print structs too.
    let struct_area = area_calculator_struct(&rectangle);
    println!("The area of the rectangle using the struct method is {}  square meters. and the rectangle is {:?}",struct_area,rectangle);

    // Calculating using struct methods
    impl Rectangle {
        fn area(&self) -> u32{
            self.width *self.height
        }
    } 

    let method_area = rectangle.area();
    println!("The area of the rectangle using the struct method is {}  square meters. and the rectangle is {:?}",method_area,rectangle);

    // There is another way to print out the structs using dbg, but dbg takes ownership and you should understand that.

    dbg!(&rectangle);


    // Since it takes owner ship, it can be assigned to variables too. Try to use references inside dbg for ownership issues.
    let rectangle2 = Rectangle{
        height:dbg!(80),
        width:180
    };

    let method2_area = rectangle2.area();
    dbg!(&method2_area);

  
}


fn area_calculator(height:u32, width:u32) -> u32{
    height*width
}

fn area_calculator_tuple(dimensions:(u32,u32)) -> u32{
    dimensions.0*dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

fn area_calculator_struct(rectangle:&Rectangle) -> u32{
    rectangle.height* rectangle.width
}
