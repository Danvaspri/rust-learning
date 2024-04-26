use std::sync::{Arc, Mutex};
use std::thread;
fn main() {
    //mutex is mutual exclusion.
    //1. acquire the lock before muting data
    //2. release lock when you're done with the data. So other threads can use it.
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); //this blocks the current thread until the lock is
                                         //acquired

        *num = 8;
    } //when the lock goes out of scope, rust calls the drop method to release the lock
      //automatically so we don't have to.
    println!("{:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{}", *num);
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
