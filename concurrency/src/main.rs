use std::thread;
use std::sync::mpsc;
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

    // Message passing between threads ** Don not communicate by sharing memory, instead share memory by communicating.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");


}
