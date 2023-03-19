// Threads general
use std::thread;
use std::time::Duration;
// Communication between threads
use std::sync::mpsc;


fn main() {
    // Creating a new thread
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Waiting for all threads to finish
    handle1.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Moving a closure to a new thread
    let v = vec![1, 2, 3];
    let handle2 = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
    });
    handle2.join().unwrap();

    // Communication between threads
    // Sending a message
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // Receiving a message
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
