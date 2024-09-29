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

    // let add_one_v3 = |x|       {x+1};

    // let add_one_v4 = |x|  x+1;

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




    
}
