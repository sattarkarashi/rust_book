fn main() {
    // It is time to start collections, collections are saved in heap. We start by vectors which can hold any lengths of the same type values.

    let mut v:Vec<i32> = Vec::new();
    // For convenience Rust provides vec! macro for defining vectors and giving it value
    let mut v2 = vec![1,2,5];

    // updating vector: to be able to update vectors, they should be mutable.
    v.push(15);
    v2.push(8);

    println!("{:?} and {:?}",v,v2);

    // Two ways to access values in vectors

    let second_value: &i32 = &v2[1];
    println!("The second value is {second_value}");

    let third_value: Option<&i32> = v2.get(2);
    match third_value {
        Some(third) => println!("The third value is {third}"),
        None => println!("There is no third value"),
    }

    // if we try to get an not existing value, the get wont panic and it will return None but the the first indexing panics.

    // let does_not_exist = v2[1000]; This panics
    let does_not_exist = v2.get(1000);


    
}
