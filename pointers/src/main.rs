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

    // Using deref for smart pointers
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);

    // Using Box<T> instead

    let x = 5;
    let y = Box::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);

    // Let's build a Box<T> and show how is different from references

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x:T) -> MyBox<T>{
            MyBox(x)
        }
    }

    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5,x);
    assert_eq!(5,*y);

    // *y actually implements the *y.deref() in practice.

    // Implicit Deref Coercion
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let m = MyBox::new(String::from("Sato"));
    hello(&m);

    // Drop trait : what happens when a value goes out of scope.
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self){
            println!("Dropping CustomSmartPointer with data {}!", self.data);
        }
    }

    let c = CustomSmartPointer {
        data: String::from("Sato is going forward.")
    };

    let d = CustomSmartPointer {
        data: String::from("Sato is learning.")
    };

    println!("CustomSmartPointers created.");

    // Note that we didn't need to explicitly call the drop method, it will call whenever the values go out of scope.

    // We can call the drop method explicitly, because it ends up in double free error, so to force drop on a value we should import the drop from std::mem::drop
    use std::mem::drop;

    drop(c);

    // Rc<T> reference counted smart pointer





}
