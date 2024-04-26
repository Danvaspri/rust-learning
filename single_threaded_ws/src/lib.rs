use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
enum Message {
    NewJob(Job),
    Terminate,
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers {
            println!("Shutting down worker: {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
type Job = Box<dyn FnOnce() + Send + 'static>;
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        let (sender, reciever) = mpsc::channel();

        let reciever = Arc::new(Mutex::new(reciever));
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&reciever)));
        }
        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

struct Worker {
    id: usize,

    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = reciever.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} is executing a job", id);
                    job();
                }

                Message::Terminate => {
                    println!("Worker {} was told to terminate", id);
                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
