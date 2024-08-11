#![allow(unused_variables)]
use std::io::Error;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

/// ## Message
///
/// Enumeracion de mensajes que se pueden enviar a los workers
///
/// ### Variantes
/// - `NewJob`: nuevo trabajo
/// - `Terminate`: terminar el worker
///
enum Message {
    NewJob(Job),
    Terminate,
}

/// ## ServerPool
///
/// Estructura que representa un pool de workers
///
/// ### Atributos
/// - `workers`: threads workers
/// - `sender`: canal de comunicacion
///
pub struct ThreadPool {
    workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Message>,
}

trait FnBox {
    /// ## call_box
    ///
    /// Ejecuta la funcion
    ///
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
    /// ## build
    ///
    /// Construye un pool de workers
    ///
    /// ### Parametros
    /// - `size`: cantidad de workers
    ///
    /// ### Retorno
    /// - `Result<ServerPool, Error>`:
    ///   - Ok: pool de workers
    ///   - Err: error al construir el pool de workers
    ///
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

        Ok(ThreadPool { workers, sender })
    }

    /// ## execute
    ///
    /// Ejecuta una funcion en un worker
    ///
    /// ### Parametros
    /// - `f`: funcion a ejecutar
    ///
    /// ### Retorno
    /// - `Result<(), Error>`:
    ///     - Ok: funcion ejecutada
    ///     - Err: error al ejecutar la funcion
    ///
    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match self.sender.send(Message::NewJob(job)) {
            Ok(a) => Ok(()),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::Other,
                "El thread no pudo enviar el mensaje al worker",
            )),
        }
    }
}

impl Drop for ThreadPool {
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

impl Clone for ThreadPool {
    fn clone(&self) -> Self {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(self.workers.len());

        for id in 0..self.workers.len() {
            workers.push(WorkerThread::new(Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
}

/// ## WorkerThread
///
/// Estructura que representa un worker
///
/// ### Atributos
/// - `thread`: thread del worker
struct WorkerThread {
    thread: Option<thread::JoinHandle<()>>,
}

impl WorkerThread {
    /// ## new
    ///
    /// Crea un nuevo worker
    ///
    /// ### Parametros
    /// - `receiver`: receptor de mensajes
    ///
    /// ### Retorno
    /// - `WorkerThread`: worker creado
    ///
    fn new(receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> WorkerThread {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    job.call_box();
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
