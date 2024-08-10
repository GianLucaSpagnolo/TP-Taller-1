use logger::logger_handler::Logger;
use native_tls::{TlsConnector, TlsStream};
use std::error::Error;
use std::net::TcpStream;
use std::thread::sleep;

// srv_name = certificated name of the address ip
pub fn connect(
    addres: &str,
    srv_name: &str,
    keep_alive: &u16,
    client_id: &String,
    logger: &Logger,
) -> Result<TlsStream<TcpStream>, Box<dyn Error>> {
    let tls_connector = TlsConnector::new()?;

    let mut tcp_stream;
    let duration = *keep_alive as u64;
    loop {
        tcp_stream = TcpStream::connect(addres);
        match tcp_stream {
            Ok(stream) => return Ok(tls_connector.connect(srv_name, stream)?),
            Err(_) => {
                let msg = "Error al intentar conectar con el servidor. Se reintentara en "
                    .to_string()
                    + &duration.to_string()
                    + " segundos.";
                logger.log_event(&msg, client_id);
                sleep(std::time::Duration::from_secs(duration));
            }
        }
    }
}
