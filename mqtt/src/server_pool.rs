#![allow(unused_variables)]
use std::io::Error;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::actions::MqttActions;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ServerPool {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    fn call_box(self: Box<Self>) -> Result<MqttActions, Error>;
}

impl<F: FnOnce() -> Result<MqttActions, Error>> FnBox for F {
    fn call_box(self: Box<F>) -> Result<MqttActions, Error> {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ServerPool {
    pub fn build(size: usize) -> Result<Self, Error> {
        if size == 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "La cantidad de workers debe ser mayor a 0",
            ));
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(WorkerThread::new(Arc::clone(&receiver)));
        }

        Ok(ServerPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() -> Result<MqttActions, Error> + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.send(Message::NewJob(job)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::Other,
                "El thread no pudo enviar el mensaje al worker",
            )),
        }
    }
}

impl Drop for ServerPool {
    fn drop(&mut self) {
        // Se envia un mensaje de terminacion a cada worker
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // Se espera a que cada worker termine
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct WorkerThread {
    thread: Option<thread::JoinHandle<()>>,
}

impl WorkerThread {
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> WorkerThread {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    let _ = job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        WorkerThread {
            thread: Some(thread),
        }
    }
}
