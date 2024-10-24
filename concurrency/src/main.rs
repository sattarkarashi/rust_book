use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::{Arc,Mutex};
use std::rc::Rc;

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

    // The following code ends in compile error because send takes ownership
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("Val is {val}");
    // });

    // Sending multiple values
    let (tx, rx) = mpsc::channel();
    thread::spawn(move|| {
        let vals = vec![String::from("Hi"),String::from("from"),String::from("the"),String::from("thread")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
   

    for received in rx {
        println!("Got: {received}");
    }

    // Creating multiple producers
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move|| {
        let vals = vec![String::from("Hi"),String::from("from"),String::from("the"),String::from("thread")];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move|| {
        let vals = vec![String::from("More"),String::from("messages"),String::from("for"),String::from("you")];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // Mutex: one thread at time
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}",m);

    // Share mutex between multiple threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();

    }

    println!("Result: {}", *counter.lock().unwrap());

    // Atomic reference counting with Arc<T>





}
