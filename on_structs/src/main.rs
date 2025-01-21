fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username:String::from("Sato"),
        sign_in_count: 1,
        active: true
    };
    println!("User1: {}", user1.username);

    let user2 = User {
        username: String::from("nobody"),
        ..user1
    };

    println!("User2: {}", user2.username);

    //tuple structs
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    // a rectangle struct
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height:u32
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold_rectange(&self, other: &Rectangle) -> bool{
            self.width > other.width && self.height > other.height
        }

        // this is an associated function that doens't get self and it will be called by :: operator
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }

    }

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!("The area of the rectangle is {}", rect1.area());
    println!("The rectangle can hold another rectangle: {}", rect1.can_hold_rectange(&rect2));
    println!("The square is: {:?}", Rectangle::square(10));
}
