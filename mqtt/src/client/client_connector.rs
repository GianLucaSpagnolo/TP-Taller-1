use native_tls::{Error, TlsConnector, TlsStream};
//use std::io::{Read, Write};
use std::net::TcpStream;
/*
struct TlsClientConnector {
    tls_stream: TlsStream<TcpStream>,
}

impl TlsClientConnector {
    // manejar unwraps:
    // srv_name = certificated name of the address ip
    fn connect(addres: &str, srv_name: &str) -> Result<Self, Error> {
        let tls_connector = TlsConnector::new()?;
        let tcp_stream = TcpStream::connect(addres).unwrap();
        let tls_stream = tls_connector
            .connect(srv_name, tcp_stream).unwrap();

        Ok(Self {
            tls_stream,
        })
    }

    fn send(&mut self, msg: &str) -> Result<(), std::io::Error>{
        self.tls_stream.write_all(msg.as_bytes())
    }

    fn receive(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.tls_stream.read(buf)
    }

    fn close(mut self) -> Result<(), std::io::Error>{
        self.tls_stream.shutdown()
    }
}
*/

// srv_name = certificated name of the address ip
pub fn connect(addres: &str, srv_name: &str) -> Result<TlsStream<TcpStream>, Error> {
    let tls_connector = TlsConnector::new()?;
    let tcp_stream = TcpStream::connect(addres).unwrap();
    Ok(tls_connector
        .connect(srv_name, tcp_stream).unwrap())
}