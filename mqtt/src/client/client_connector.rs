use native_tls::{TlsConnector, TlsStream};
use std::error::Error;
use std::net::TcpStream;

// srv_name = certificated name of the address ip
pub fn connect(addres: &str, srv_name: &str) -> Result<TlsStream<TcpStream>, Box<dyn Error>> {
    let tls_connector = TlsConnector::new()?;
    let tcp_stream = TcpStream::connect(addres)?;
    Ok(tls_connector.connect(srv_name, tcp_stream)?)
}
