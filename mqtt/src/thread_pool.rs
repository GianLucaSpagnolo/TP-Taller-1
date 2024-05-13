#![allow(unused_variables)]
use std::io::Error;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::server::ServerActions;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>) -> Result<ServerActions, Error>;
}

impl<F: FnOnce() -> Result<ServerActions, Error>> FnBox for F {
    fn call_box(self: Box<F>) -> Result<ServerActions, Error> {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, Error> {
        if size == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Thread pool size cannot be zero",
            ));
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() -> Result<ServerActions, Error> + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.send(Message::NewJob(job)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(std::io::ErrorKind::Other, "Thread pool is dead")),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);

                    let _ = job.call_box();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
