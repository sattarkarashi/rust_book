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
}
