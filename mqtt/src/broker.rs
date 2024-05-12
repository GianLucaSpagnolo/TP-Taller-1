use mqtt::{
    config::{Config, ServerConfig},
    server::{Server, ServerActions},
};

use std::{
    env,
    io::Error,
    net::{TcpListener, TcpStream},
    sync::{mpsc, Arc, Mutex},
    thread,
};

struct WorkerThread {
    _id: usize,
    _thread: thread::JoinHandle<()>,
}

impl WorkerThread {
    fn new(_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> WorkerThread {
        let _thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {_id} got a job; executing.");

            let _ = job();
        });

        WorkerThread { _id, _thread }
    }
}

type Message = Box<dyn FnOnce() -> Result<ServerActions, Error> + Send + 'static>;

pub struct ThreadPool {
    _workers: Vec<WorkerThread>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn build(size: usize) -> Result<ThreadPool, Error> {
        if size <= 0 {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "El tamaño del pool debe ser mayor a 0",
            ));
        }

        // crea un canal para enviar mensajes a los workers (como si fuera un pipe)
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut _workers = Vec::with_capacity(size);

        for _ in 0..size {
            let id = _workers.len();
            _workers.push(WorkerThread::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool { _workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() -> Result<ServerActions, Error> + Send + 'static,
    {
        let message = Box::new(f);

        match self.sender.send(message) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::new(
                std::io::ErrorKind::Other,
                format!("No se pudo enviar el mensaje al worker: {:?}", e),
            )),
        }
    }
}

fn handle_client(
    server: Arc<Mutex<Server>>,
    client_stream: TcpStream,
    pool: &ThreadPool,
) -> Result<ServerActions, Error> {
    match pool.execute(move || -> Result<ServerActions, Error> {
        server.lock().unwrap().process_packet(client_stream)
    }) {
        Ok(_) => Ok(ServerActions::ConnectionEstablished),
        Err(e) => Err(e),
    }
}

fn run_listener(server: Server, listener: &TcpListener) -> Result<ServerActions, Error> {
    let maximum_clients = 4; //server.config
    let pool = ThreadPool::build(maximum_clients)?;

    let server_ref = Arc::new(Mutex::new(server));

    for client_stream in listener.incoming() {
        let shared_server = server_ref.clone();

        handle_client(shared_server, client_stream?, &pool)?;
    }

    Err(Error::new(
        std::io::ErrorKind::Other,
        "No se pudo recibir el paquete",
    ))
}

// le devuelve el paquete al servidor
// el servidor lo pasa al logger
// el logger le pide traduccion al protocolo
pub fn start_server(config: ServerConfig) -> Result<ServerActions, Error> {
    let server = Server::new(config);

    let listener = match TcpListener::bind(server.get_address()) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };

    // corre un listener en un thread pool
    run_listener(server, &listener)
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos incorrecta - debe pasarse el archivo de configuracion del servidor",
        ));
    }

    let config_path = &args[1];

    let config = ServerConfig::from_file(String::from(config_path))?;

    let addr = config.get_socket_address();

    match start_server(config) {
        Ok(_) => println!("Corriendo servidor en {:?}", addr),
        Err(e) => println!("Error en el server: {:?}", e),
    }

    Ok(())
}
