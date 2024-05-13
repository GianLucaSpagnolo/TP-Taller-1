use std::{io::Error, sync::{mpsc, Arc, Mutex}, thread};

use crate::server::ServerActions;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}


struct WorkerThread {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl WorkerThread {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> WorkerThread {
        let thread = thread::spawn(move || 
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);

                        match job() {
                            Ok(_) => {
                                println!("Packet procesado en {} finished the job.", id);
                            },
                            Err(e) => {
                                println!("Worker {} failed to execute the job: {:?}", id, e);
                            }
                        
                        };
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);

                        break;
                    },
                }
            }
        );

        WorkerThread { id, thread: Some(thread) }
    }
}


type Job = Box<dyn FnOnce() -> Result<ServerActions, Error> + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, Error> {
        if size == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "El tamaño del pool debe ser mayor a 0",
            ));
        }

        // crea un canal para enviar mensajes a los workers (como si fuera un pipe)
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            let id = workers.len();
            workers.push(WorkerThread::new(id, Arc::clone(&receiver)));
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
            Err(e) => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("No se pudo enviar el mensaje al worker: {:?}", e),
            )),
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
