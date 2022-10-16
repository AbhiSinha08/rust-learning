use std::thread::{Builder, JoinHandle};
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::process;

struct Job {
    function: Box<dyn FnOnce() -> () + Send>
}

struct Worker {
    thread: Option<JoinHandle<()>>,
    id: usize
}

impl Worker {
    fn new(id: usize, reciever: Arc<Mutex<Receiver<Job>>>) -> Option<Worker> {
        let builder = Builder::new();
        let handle = builder.spawn(move || {
            loop {
                let reciever = match reciever.lock() {
                    Ok(r) => r,
                    Err(_) => {
                        println!("Shutting down worker {}", id);
                        break
                    }
                };
                let job = match reciever.recv() {
                    Ok(j) => j.function,
                    Err(_) => {
                        println!("Shutting down worker {}", id);
                        drop(reciever);
                        break
                    }
                };
                drop(reciever);
                job();
            }
        });
        if let Err(_) = handle {
            eprintln!("Cannot create thread id: {}", id);
            return None;
        }

        Some(Worker {
            thread: Some(handle.unwrap()),
            id
        })
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (tx, rx) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);
        let rx = Arc::new(Mutex::new(rx));

        for i in 0..size {
            let rx = Arc::clone(&rx);
            let worker = match Worker::new(i, rx) {
                Some(w) => w,
                None => process::exit(1)
            };
            workers.push(worker)
        }

        ThreadPool {
            workers,
            sender: Some(tx)
        }
    }

    pub fn add<F>(&self, f: F)
        where F: FnOnce() -> () + 'static + Send
    {
        let job = Job {
            function: Box::new(f)
        };

        if let Err(_) = self.sender.as_ref().unwrap().send(job) {
            eprintln!("Unable to send the request to a thread to process");
        };
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
