fn main() {
    let mut counter = 0;
    'counting_up: loop {
        let mut decounter = 10;
        println!("Counter: {}", counter);

        'counting_down: loop {
            if counter == decounter {
                break 'counting_down;
            }
            println!("Decounter: {}", decounter);
            decounter -= 1;
        }
        counter += 1;
        if counter == 10 {
            break 'counting_up;
        }
    }
}
