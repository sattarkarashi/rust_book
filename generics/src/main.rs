fn main() {
    // First start to know how to recognize duplicated codes that can use generics

    // find the largest number in a vector
    let number_list = vec![19,23,21,98,39];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number
        }
    }
    println!("The largest number is {largest}");

    // So if we try to do this logic for another vector, it will take much space and much duplications will occur. So using a function will solve the duplication problem.

    fn get_the_largest_number(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item
            }
        }
        println!("The largest number is {largest}");
        return largest;

    }

    let first_list = vec![43,89,27,89,32];
    let second_list = vec![99,32,88,44,56];

    get_the_largest_number(&first_list);
    get_the_largest_number(&second_list);

    // Now we reduced the duplication using the function

    // Let's write a similar function for chars
    fn get_the_largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item
            }
        }
        println!("The largest char is {largest}");
        return largest;
    }

    let chars = vec!['e', 'l', 'f','a','n'];
    get_the_largest_char(&chars);

    // Using generics, we can use a type for a single function to work for both types:
    fn get_the_largest_generic<T: std::fmt::Display + std::cmp::PartialOrd>(list: &[T]) -> &T {

        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item
            }
        }
        println!("The largest val is {largest}");
        return largest;
    }

    let chars2 = vec!['d','q','z'];
    let numbers2 = vec![23,19,36,96];

    get_the_largest_generic(&chars2);
    get_the_largest_generic(&numbers2);

    // Now it works for both scenarios and we have removed quiet a good amount of duplications.

    // Generics inside structs
    struct MyCordinates<T> {
        x:T,
        y:T
    }

    let int_cordinates = MyCordinates {
        x:2,
        y:3
    };

    let float_cordinates = MyCordinates {
        x:2.3,
        y:3.0
    };

    // To use multiple types:
    struct Point <T, U> {
        x:T,
        y:U
    }

    let int_float = Point {
        x:2.4,
        y:19
    };

    // We can implement generics for methods too
    impl<T> MyCordinates<T>{
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = MyCordinates{
        x:5,
        y:19
    };

    println!("p.x = {}", p.x());
}
