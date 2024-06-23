use native_tls::{Error, HandshakeError, Identity, TlsAcceptor, TlsStream};
use std::fs::File;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct TlsServerConnector {
    listener: TcpListener,
    acceptor: TlsAcceptor,
}

impl TlsServerConnector {
    // manejar unwraps
    pub fn initialize(
        cert_path: &str,
        pass: &str,
        address: &str,
    ) -> Result<TlsServerConnector, Error> {
        let mut file = File::open(cert_path).unwrap();
        let mut buf: Vec<u8> = vec![];

        file.read_to_end(&mut buf).unwrap();

        let identity = Identity::from_pkcs12(&buf, pass)?;

        let listener = TcpListener::bind(address).unwrap();
        let acceptor = TlsAcceptor::new(identity).unwrap();

        Ok(TlsServerConnector { listener, acceptor })
    }

    pub fn get_listener(&self) -> Result<TcpListener, std::io::Error> {
        self.listener.try_clone()
    }

    pub fn accept_tls_connection(
        &self,
        stream: TcpStream,
    ) -> Result<TlsStream<TcpStream>, HandshakeError<TcpStream>> {
        self.acceptor.accept(stream)
    }
}
