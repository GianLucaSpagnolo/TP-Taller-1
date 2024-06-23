use std::io::Error;
use std::net::TcpStream;
//use std::str::Lines;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use logger::logger_handler::Logger;

use crate::config::{mqtt_config::Config, server_config::ServerConfig};
use crate::logging::actions::MqttActions;
use crate::logging::server_actions::MqttServerActions;
use crate::mqtt_packets::headers::fixed_header::PacketFixedHeader;
use crate::mqtt_packets::packet::generic_packet::{get_packet, PacketReceived, Serialization};
use crate::mqtt_packets::packets::pingresp::PingResp;

use super::server_connector::TlsServerConnector;
use super::server_handlers::{
    connect_handler, disconnect_handler, publish_handler, subscribe_handler, unsubscribe_handler,
};

use super::server_network::ServerNetwork;
use super::server_register::SessionRegister;

/// ## MqttServer
///
/// Estructura que representa un servidor MQTT
///
/// ### Atributos
/// - `config`: Configuración del servidor
/// - `sessions`: Sesiones del servidor
/// - `connect_received`: Bandera que indica si se recibió un paquete de conexión
///
pub struct MqttServer {
    pub config: ServerConfig,
    pub register: SessionRegister,
    pub network: ServerNetwork,
    pub connect_received: bool,
}

impl Clone for MqttServer {
    fn clone(&self) -> Self {
        MqttServer {
            config: self.config.clone(),
            register: self.register.clone(),
            network: self.network.clone(),
            connect_received: self.connect_received,
        }
    }
}

/// ## message_catcher
///
/// Función que maneja los paquetes recibidos por el servidor
///
/// ### Parametros
/// - `stream`: Stream de la conexión
/// - `sender`: Sender del servidor (envia los mensajes para que sean procesados)
///
/// ### Retorno
/// - `Result<(), Error>`: Resultado de la operación
///
pub fn message_catcher(
    mut stream: TcpStream,
    sender: Arc<Mutex<Sender<(PacketReceived, TcpStream)>>>,
) -> Result<(), Error> {
    let sender = sender.lock().unwrap().clone();

    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    match get_packet(
        &mut stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    ) {
        Ok(pack) => match sender.send((pack, stream)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Error al enviar el paquete",
            )),
        },
        Err(e) => Err(e),
    }
}

/// ## client_handler
///
/// Función que maneja las conexiones de los clientes
///
/// ### Parametros
/// - `listener`: Listener del servidor
/// - `sender`: Sender del servidor (envia los mensajes para que sean procesados)
///
fn client_handler(
    //listener: TcpListener,
    server_connector: TlsServerConnector,
    sender: Arc<Mutex<Sender<(PacketReceived, TcpStream)>>>,
) -> Result<(), Error> {
    /*
    for client_stream in listener.incoming() {
        let stream = client_stream?.try_clone()?;
        let sender_clone = Arc::clone(&sender);
        thread::spawn(move || -> Result<(), Error> {
            loop {
                // Manejo de paquetes, cuando se recibe un paquete se envia al procesador de mensajes
                message_catcher(stream.try_clone()?, sender_clone.clone())?;
            }
        });
    }
    */
    let listener = server_connector.get_listener().unwrap();
    let srv_cpy = Arc::new(server_connector);

    for client_stream in listener.incoming() {
        let client_stream = match client_stream {
            Err(e) => return Err(e),
            Ok(client_stream) => client_stream,
        };

        let srv = srv_cpy.clone();
        //let srv = srv_cpy.clone();
        let mut stream = srv
            .accept_tls_connection(client_stream)
            .expect("error al aceptar conexion");

        let stream = stream.get_mut();

        let stream = stream.try_clone()?;
        let sender_clone = Arc::clone(&sender);
        thread::spawn(move || -> Result<(), Error> {
            loop {
                // Manejo de paquetes, cuando se recibe un paquete se envia al procesador de mensajes
                message_catcher(stream.try_clone()?, sender_clone.clone())?;
            }
        });
    }

    Ok(())
}

impl MqttServer {
    /// ### new
    ///
    /// Crea un nuevo servidor MQTT
    ///
    /// ### Parametros
    /// - `config`: Configuración del servidor
    ///
    pub fn new(config: ServerConfig) -> Self {
        let register = SessionRegister::new(config.db_path.clone());
        let network = ServerNetwork::default();

        MqttServer {
            config,
            register,
            network,
            connect_received: false,
        }
    }

    /// ### start_server
    ///
    /// Inicia el servidor MQTT
    ///
    /// ### Retorno
    /// - `Result<(), Error>`: Resultado de la operación
    ///
    pub fn start_server(self, logger: Logger) -> Result<(), Error> {
        let id = self.config.general.id.clone();
        let logger_cpy = logger.clone();
        

        self.register.log_sessions(
            &self.config.general.id,
            &self.config.general.log_in_term,
            &logger,
        );
        /*
        let listener = match TcpListener::bind(self.config.get_socket_address()) {
            Ok(lis) => lis,
            Err(e) => {
                logger_cpy.log_event(
                    &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                    &self.config.general.id,
                );
                logger_cpy.close();
                logger.close();
                return Err(e);
            }
        };
        */
        let address = self.config.get_socket_address().to_string();
        let cert_path = self.config.general.cert_path.clone();
        let cert_pass = self.config.general.cert_pass.clone();
        let server_connector =
            match TlsServerConnector::initialize(&cert_path, &cert_pass, &address) {
                Ok(srv) => srv,
                Err(e) => {
                    logger_cpy.log_event(
                        &("Error al conectar con servidor: ".to_string() + &e.to_string()),
                        &self.config.general.id,
                    );
                    logger_cpy.close();
                    logger.close();
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("TLS error de conexion: {}", e),
                    ));
                }
            };

        //let listener = server_connector.get_listener()?;

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let receiver = Arc::new(Mutex::new(receiver));

        // Iniciando el procesador de mesages que recibe el servidor
        self.server_listener_messages(Arc::clone(&receiver), logger_cpy.clone());

        // Iniciando el listener de conexiones que recibe el servidor dentro de un thread pool
        // agregar tls connector
        client_handler(server_connector, Arc::clone(&sender))?;

        logger.log_event(
            &("Cerrando servidor ... no se reciben mas paquetes".to_string()),
            &id,
        );

        logger_cpy.close();
        logger.close();
        Err(Error::new(
            std::io::ErrorKind::Other,
            "No se pudo recibir el paquete",
        ))
    }

    /// ### process_messages
    ///
    /// Procesa los mensajes recibidos por el servidor
    ///
    /// ### Parametros
    /// - `receiver`: Receiver de los mensajes
    ///
    fn process_messages(
        &mut self,
        receiver: Arc<Mutex<Receiver<(PacketReceived, TcpStream)>>>,
        logger: &Logger,
    ) -> Result<MqttServerActions, Error> {
        let (pack, mut stream) = receiver.lock().unwrap().recv().unwrap();
        match pack {
            PacketReceived::Connect(connect_pack) => {
                connect_handler::stablish_connection(self, stream, *connect_pack, logger)
            }
            PacketReceived::Disconnect(disconnect_pack) => {
                disconnect_handler::receive_disconnect(self, *disconnect_pack, logger)
            }
            PacketReceived::Publish(pub_packet) => {
                publish_handler::resend_publish_to_subscribers(self, stream, *pub_packet, logger)
            }
            PacketReceived::Subscribe(sub_packet) => {
                subscribe_handler::add_subscriptions(self, stream, *sub_packet, logger)
            }
            PacketReceived::Unsubscribe(unsub_packet) => {
                unsubscribe_handler::remove_subscriptions(self, stream, *unsub_packet, logger)
            }
            PacketReceived::PingReq(_) => {
                MqttServerActions::ReceivePingReq.log_action(
                    &self.config.general.id,
                    logger,
                    &self.config.general.log_in_term,
                );
                PingResp.send(&mut stream)?;
                Ok(MqttServerActions::SendPingResp)
            }
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "Server - Paquete recibido no es válido",
            )),
        }
    }

    /// ### server_listener_messages
    ///
    /// Inicia el listener de mensajes del servidor
    ///
    /// ### Parametros
    /// - `receiver`: Receiver de los mensajes (procesados por el client_handler)
    /// - `log_path`: Path del log
    ///
    fn server_listener_messages(
        mut self,
        receiver: Arc<Mutex<Receiver<(PacketReceived, TcpStream)>>>,
        logger: Logger,
    ) {
        let logger_cpy = logger.clone();
        thread::spawn(move || -> Result<(), Error> {
            let log_2 = logger_cpy.clone();
            loop {
                let log = log_2.clone();
                match self.process_messages(Arc::clone(&receiver), &log) {
                    Ok(a) => {
                        a.log_action(
                            &self.config.general.id,
                            &log,
                            &self.config.general.log_in_term,
                        );
                    }
                    Err(e) => {
                        logger_cpy.log_event(
                            &("Error al procesar el mensaje: ".to_string() + &e.to_string()),
                            &self.config.general.id,
                        );
                        logger_cpy.close();
                        return Err(e);
                    }
                };
                log.close();
            }
        });
        logger.close();
    }
}
