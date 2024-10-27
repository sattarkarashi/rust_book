fn main() {
    pub struct AverageCollection {
        list: Vec<i32>,
        average: f32,
    }

    impl AverageCollection {
        pub fn add(&mut self, value:i32){
            self.list.push(value);
            self.update_average();
        }

        pub fn remove(&mut self) -> Option<i32> {
            let result = self.list.pop();
            match result {
                Some(value) => {
                    self.update_average();
                    Some(value)
                }, 
                None => None,
            }
        }

        pub fn average(&self) -> f32{
            self.average
        }

        fn update_average(&mut self){
            let total: i32 = self.list.iter().sum();
            self.average = total as f32/self.list.len() as f32;
        }
    }

    // The option to use pub or not enables encapsulation in Rust. And considering structs and enum which can contain data, 
    // and impl which contains methods, we can consider Rust OOP.

    // Inheritance as a type system and as code sharing: Rust uses trait objects instead of inheritance. Inheritance is risky because it risks overexposure.

    

}
