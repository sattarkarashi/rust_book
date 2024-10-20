use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi num {i} from spawned thread");
        };
        thread::sleep(Duration::from_millis(1));

    });


    for i in 1..5 {
        println!("Hi num {i} from main thread");
        thread::sleep(Duration::from_millis(1));

    };

    
    handle.join().unwrap();

    // Moving a values into another thread and using them

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}",v);
    });

    handle.join().unwrap();

    // Message passing between threads

}
