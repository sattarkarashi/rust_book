enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    // Using Box<T> to store data on the heap
    let b = Box::new(5);
    println!("b = {b}");

    // Using box for an i32 is not helpful, we would need it for some scenarios like cons lists which are of recursive types
    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }

    // let list = Cons(1,Cons(2,Cons(3,Nil)));

    // Computing the size of a Non-Recursive type: The amount of the most space-ful variant is determining the space we need.

    // enum Message{
    //     Quit,
    //     Move{x:i32, y:i32},
    //     Write(String),
    //     ChangeColor(i32,i32,i32)
    // }

    // Using Box<T> to get a recursive type with a known Size

    

    let list = Cons(1, Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));



}
