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
}
