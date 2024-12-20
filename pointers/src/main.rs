// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};

// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }
// use crate::List::{Cons, Nil};
use std::rc::Rc;
use std::cell::RefCell;

 // allowing multiple owners of mutable data with Rc and Refcell

//  #[derive(Debug)]
//  enum List {
//      Cons(Rc<RefCell<i32>>, Rc<List>),
//      Nil,
//  }

#[derive(Debug)]
 enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
 }

 impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
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

    

    // let list = Cons(1, Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));

    // // Using deref for smart pointers
    // let x = 5;
    // let y = &x;
    // assert_eq!(5,x);
    // assert_eq!(5,*y);

    // // Using Box<T> instead

    // let x = 5;
    // let y = Box::new(x);
    // assert_eq!(5,x);
    // assert_eq!(5,*y);

    // // Let's build a Box<T> and show how is different from references

    // struct MyBox<T>(T);
    // impl<T> MyBox<T> {
    //     fn new(x:T) -> MyBox<T>{
    //         MyBox(x)
    //     }
    // }

    // use std::ops::Deref;

    // impl<T> Deref for MyBox<T> {
    //     type Target = T;
    //     fn deref(&self) -> &Self::Target {
    //         &self.0
    //     }
    // }

    // let x = 5;
    // let y = MyBox::new(x);
    // assert_eq!(5,x);
    // assert_eq!(5,*y);

    // // *y actually implements the *y.deref() in practice.

    // // Implicit Deref Coercion
    // fn hello(name: &str) {
    //     println!("Hello, {name}!");
    // }

    // let m = MyBox::new(String::from("Sato"));
    // hello(&m);

    // // Drop trait : what happens when a value goes out of scope.
    // struct CustomSmartPointer {
    //     data: String,
    // }

    // impl Drop for CustomSmartPointer {
    //     fn drop(&mut self){
    //         println!("Dropping CustomSmartPointer with data {}!", self.data);
    //     }
    // }

    // let c = CustomSmartPointer {
    //     data: String::from("Sato is going forward.")
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("Sato is learning.")
    // };

    // println!("CustomSmartPointers created.");

    // // Note that we didn't need to explicitly call the drop method, it will call whenever the values go out of scope.

    // // We can call the drop method explicitly, because it ends up in double free error, so to force drop on a value we should import the drop from std::mem::drop
    // use std::mem::drop;

    // drop(c);

    // Rc<T> reference counted smart pointer

    
    
    

    // let a = Rc::new(Cons(5,
    //         Rc::new(Cons(10,
    //         Rc::new(Nil)))));

    // println!("Count after creating a = {}", Rc::strong_count(&a));

    // let b = Cons(3, Rc::clone(&a));
    // println!("Count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("Count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("Count after c goes out of scope = {}", Rc::strong_count(&a));

    // Rc::clone just increments the references instead of deep copying

    // RefCall: The invariants are enforced at runtime while with box<T> it was enforced in compile time.

    // Interior mutability: the below code will result in an error because you are not allowed to get a mutable borrow of an immutable value:

    // let x = 5;
    // let y = &mut x;

    // let value = Rc::new(RefCell::new(5));
    // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // *value.borrow_mut() += 10;
    // println!("a after = {:?}", a);
    // println!("b after = {:?}", b);
    // println!("c after = {:?}", c);

    // Creating reference cycles
    
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // The following println will overflow the stack!
    println!("a next item = {:?}", a.tail());


    


 

    
}
