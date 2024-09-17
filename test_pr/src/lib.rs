// Let's write a test for a previous struct we worked on:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn adds_two(a:i32) -> i32 {
    a + 2
}

pub fn greetings(name: &str) -> String {
    format!("Hello {name}!")
}

pub struct Guess {
    value:i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 300 {
            panic!("The value should be between 1 and 100 but got {}", value);
        }
        Guess{value}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width:8,
            height:12
        };

        let smaller = Rectangle{
            width:6,
            height:9
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn larger_cannot_hold_smaller(){
        let larger = Rectangle {
            width:8,
            height:12
        };

        let smaller = Rectangle{
            width:18,
            height:9
        };

        assert!(!larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!(4,adds_two(2));
    }

    #[test] 
    fn it_greets(){
        let result = greetings("Sato");

        assert!(result.contains("Sato"),"Greetings didn't contain name, the value was {result}");
    }

    #[test]
    #[should_panic(expected="less than or equal to 100")]
    fn greater_than_100 (){
        Guess::new(200);
    }
}
