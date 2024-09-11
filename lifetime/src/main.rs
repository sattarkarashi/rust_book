fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}",r);

    // The above code panics because the scope of r is not valid outside the {}. Lifetimes are for preventing the reference dangling.

    // Write a function to return the longest string between two

    // fn longest(x: &str, y:&str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     }else{
    //         y
    //     }
    // }

    // The above code will hit a lifetime error. To solve it we should ensure the function about the lifetimes:

    fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
        if x.len() > y.len() {
            x
        }else{
            y
        }
    }

    let first_str = String::from("Sato is hereeeee");
    let second_str = "He is not here";

    let what_is_longest = longest(&first_str,&second_str);
    println!("The longest str is {}", what_is_longest);


}
