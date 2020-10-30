use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::Mutex;


fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // Have to move ownership of the reference to the thread.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // a Channel! Like go Channel.
    // Returns a sender (tx) and a receiver (rx) for the channel.
    let (tx, rx) = mpsc::channel();



    thread::spawn(move || {
        let val = String::from("hi");
        thread::sleep(Duration::from_millis(100));
        tx.send(val).unwrap();
    });

    // blocks until something is received.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Using a mutex
    let m = Mutex::new(5);

    {
        // when out of scope the mutex lock is released
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);


}
