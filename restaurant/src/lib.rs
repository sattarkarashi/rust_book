pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

fn deliver_orders(){}
// models can contain other models which contain a set of functions
mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
        
    }
    fn fix_order(){
        // using super in relative path
        super::deliver_orders();
    }

}


// paths absolute and relative
pub fn eat_at_reastaurant(){

    // Absolute path
    crate::front_of_house::hosting::add_to_wait_list();

    // Relative path
    front_of_house::hosting::add_to_wait_list();

    
}

// By default the contents the models are private and you should specify which sections be public by using pub

// making structs public: after defining pub struct, we need to also pub the fields we need to be public, and by default they are private.
// Making enums public: if we put a pub in front of the enum, it will make it public with all its fields unlike structs

mod back_of_the_hourse {
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit:String,
    }

    pub enum Appetizer{
        Soup, 
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast{
            Breakfast {
                toast:String::from(toast),
                seasonal_fruit: String::from("grapes"),
            }
        }
    }
}

pub fn eat_at_reastaurantt(){
    // Order a breakfast in the summer with Rye toast.
    let mut meal = back_of_the_hourse::Breakfast::summer("Rye");
    // change mind to weath
    meal.toast = String::from("Wheat");
    println!(" I'd like {} toast please", meal.toast);

    // if we try to change or use seasonal_food, we will encounter an error because it is not public.

    let order1 = back_of_the_hourse::Appetizer::Soup;
    let order2 = back_of_the_hourse::Appetizer::Salad;

}

