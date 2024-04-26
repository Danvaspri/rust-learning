use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    //By calling the join function before, the main thread will wait until it finishes before moving
    //on.
    for i in 1..5 {
        println!("hi number {} from main thread", i);
        thread::sleep(Duration::from_millis(5));
    }

    //if we were to call the join fn in here, bot fuctions will execute at the same time, in
    //parallel.

    //Now, if the thread needs a variable, like in the following example:

    let v = vec![1, 2, 3];
    //by specifying the word move, we pass the thread ownership of the closure needed, ensuring no
    //data races or invalid pointer.
    let vec_handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
}
