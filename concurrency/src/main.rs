use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi num {i} from spawned thread");
        };
        thread::sleep(Duration::from_millis(1));

    });


    for i in 1..5 {
        println!("Hi num {i} from main thread");
        thread::sleep(Duration::from_millis(1));

    };


    println!("Hello, world!");
}
