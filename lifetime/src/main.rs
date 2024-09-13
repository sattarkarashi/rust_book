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

    // let's define a new longest function

    fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // this function works because we our return only only depends on the x parameter.

    // let's check this one
    // fn longest3<'a>(x: &str, y:&str) -> &'a str {
    //     let result = String::from("Sato is here");
    //     return result.as_str();
    // }

    // This code panics, because we are trying to return the reference of result which its scope is over.

    // lifetime with structs

    let novel = String::from("Sato is here ...");
    let first_sentence = novel.split('.').next().expect("Cound not find a '.' ");
    let i = ImportantExcerpt {
        part:first_sentence,
    };
    println!("The first sentence is {}", i.part);

    // ** We need to use lifetime parameter for functions or structs that use references.

    // Some types of lifetimes are deterministic and you can find out about them by three rules and and you wont need to use 
    // lifetime parameters for them.
    // first rule is that each reference parameter gets a lifetime parameter
    // second rule is that if having one reference parameter, its lifetime applies to all the output values
    // in methods if we have several parameters and one of which is &self or & mut self, its lifetime applies to all other parameters

    // let's implement a method for ImportantExcerpt
    impl <'a> 
    ImportantExcerpt <'a> {
        fn announce_and_return_part(&self, announcement:&str) -> &str {
            println!("Attention please: {announcement}");
            self.part
        }
    }

    // As you can see we didn't use lifetime annotations for the input and output parameters of the method based on the third rule.
    

}
struct ImportantExcerpt <'a> {
    part: &'a str,
}
