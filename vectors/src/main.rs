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

    // Let's consider the following code
    let mut a_vector = vec![100, 8, 96,19];

    let sec_element = &a_vector[2];
    println!("The second element is {sec_element}.");

    a_vector.push(18);
    // The following print will result in panic because it is about vectors work. Vector puts elements in memory beside each other and sometimes it changes the memory and referencing it will result in error after change.
    //println!("The second element is {sec_element}.");

    // Iteration over vector values

    let iter_vector = vec![18,39,24,108,79,23];
    for i in &iter_vector {
        println!("{i}");
    }

    let mut iter_vector2 = vec![19,21,8,7,11,10];
    println!("The mutable vector is {:?}",iter_vector2);

    for i in &mut iter_vector2 {
        *i +=19;
    }

    println!("The mutable vector after change is {:?}",iter_vector2);



    
}
