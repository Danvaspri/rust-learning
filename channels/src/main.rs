use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("hi"),
            String::from("hello"),
            String::from("hola"),
        ];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let msgs = vec![String::from("Bonjour"), String::from("Magmuaselle")];
        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
