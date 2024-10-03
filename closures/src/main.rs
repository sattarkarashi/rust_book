use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference:Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red +=1,
                    ShirtColor::Blue => num_blue +=1,
                }
            }

            if num_red > num_blue {
                ShirtColor::Red
            }else {
                ShirtColor::Blue
            }
        }
    }

    // We're defining a closure here

#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1,giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2,giveaway2);


    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    expensive_closure(2);

    fn add_one_v1(x:u32) -> u32 {x+1}

    let add_one_v2 = |x:u32| -> u32 {x+1};

    let add_one_v3 = |x|       {x+1};

    let s = add_one_v3(5);

    let add_one_v4 = |x|  x;
    // If you don't give it value, it will end in error, because it wants to infer the type.
    let s2 = add_one_v4(String::from("Sato"));


    // Ownership and closures

    let list = vec![1,2,3,4];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    only_borrows();
    println!("After defining closure: {:?}", list);

    let mut list2 = vec![1,2,3,4];
    println!("Before defining closure in the mutable phase: {:?}", list2);

    let mut borrows_mutably = || list2.push(7);

    borrows_mutably();
    println!("After defining closure in the mutable phase: {:?}", list2);

    // Initiating this thread without move ends in an error, because the thread may start after the main thread is over and the value is no longer valid.

    thread::spawn(move || {
        println!("From thread: {:?}", list)
    }).join().unwrap();

    // Let's look at unwrap_or_else method

    // impl<T> Option<T> {
    //     pub fn unwrap_or_else<F>(self, f:F) -> T
    //     where F:FnOnce() -> T {
    //         math self {
    //             Some(x) => x,
    //             None => f(),
    //         }
    //     }
    // }

    

    
    let mut list = [Rectangle{width:10,length:18}, Rectangle{width:8,length:19},Rectangle{width:13,length:6}];
    
    list.sort_by_key(|r| r.width);
    println!("{:#?}",list);
    

    // This closure moves a value out of the environment and uses FnOnce trait
    

    let mut list2 = [Rectangle{width:10,length:18}, Rectangle{width:8,length:19},Rectangle{width:13,length:6}];
    // let mut sort_operations = vec![];

    let value = String::from("By key called");

    // list2.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}",sort_operations);

    // The above code results in error because it moves a variable out of the environment and it calls FnOnce while the sorting calls the closure multiple times which ends in error.


    // The below code shows how many times the sort_by_key calls the closure.
    let mut num_sort_operations = 0;
    list2.sort_by_key(|r| {
        num_sort_operations +=1;
        r.width
    });

    println!("{:#?}, sorted in {num_sort_operations}",list2);

    // Iterators: Iterators are lazy, meaning that they have no effect until you call methods on them.
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();

    // This is where it is called
    for val in v1_iter {
        println!("Got: {val}");
    }





    
}
