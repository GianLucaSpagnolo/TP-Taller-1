use std::collections::HashMap;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use logger::logger_handler::{create_logger_handler, Logger};

use crate::common::reason_codes::ReasonCode;
use crate::config::{mqtt_config::Config, server_config::ServerConfig};
use crate::logging::actions::MqttActions;
use crate::logging::server_actions::MqttServerActions;
use crate::mqtt_packets::headers::fixed_header::PacketFixedHeader;
use crate::mqtt_packets::packet::generic_packet::{get_packet, PacketReceived, Serialization};
use crate::mqtt_packets::packets::pingresp::PingResp;

use super::server_handlers::{
    connect_handler, disconnect_handler, publish_handler, subscribe_handler, unsubscribe_handler,
};
use super::server_session::Session;
// ver caso logger en drop ...

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
    pub sessions: HashMap<String, Session>,
    pub connect_received: bool,
}

impl Clone for MqttServer {
    fn clone(&self) -> Self {
        MqttServer {
            config: self.config.clone(),
            sessions: self.sessions.clone(),
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
    // averiguo el tipo de paquete:
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
    listener: TcpListener,
    sender: Arc<Mutex<Sender<(PacketReceived, TcpStream)>>>,
) -> Result<(), Error> {
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
        MqttServer {
            config,
            sessions: HashMap::new(),
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
    pub fn start_server(self) -> Result<(), Error> {
        let id = self.config.general.id.clone();
        let log_path = self.config.general.log_path.to_string();
        let logger = match create_logger_handler(&log_path) {
            Ok(log) => {
                log.log_event(
                    &"Logger del servidor inicializado".to_string(),
                    &self.config.general.id,
                );
                log
            }
            Err(e) => {
                //eprintln!("Error obtenido al inicializar el logger del servidor: {}", e);
                return Err(e);
            }
        };

        let logger_cpy = logger.get_logger();

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

        let (sender, receiver) = mpsc::channel();

        let sender = Arc::new(Mutex::new(sender));

        let receiver = Arc::new(Mutex::new(receiver));

        // Iniciando el procesador de mesages que recibe el servidor
        //self.server_listener_messages(Arc::clone(&receiver), log_path);

        self.server_listener_messages(Arc::clone(&receiver), logger_cpy.clone());

        // Iniciando el listener de conexiones que recibe el servidor dentro de un thread pool
        client_handler(listener, Arc::clone(&sender))?;

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
                connect_handler::stablish_connection(self, stream, *connect_pack)
            }
            PacketReceived::Disconnect(disconnect_pack) => {
                disconnect_handler::receive_disconnect(stream, *disconnect_pack)
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

// no se puede extender drop, necesita crear su logger
impl Drop for MqttServer {
    fn drop(&mut self) {
        let logger_handler = create_logger_handler(&self.config.general.log_path).unwrap();
        let logger = logger_handler.get_logger();
        for (_, session) in self.sessions.iter_mut() {
            match disconnect_handler::send_disconnect(
                &mut session.stream_connection,
                ReasonCode::Success,
            ) {
                Ok(a) => a.log_action(
                    &self.config.general.id,
                    &logger,
                    &self.config.general.log_in_term,
                ),
                Err(e) => eprintln!("Error al enviar el paquete de desconexión: {}", e),
            };
            let _ = session.disconnect();
        }

        MqttServerActions::CloseServer.log_action(
            &self.config.general.id,
            &logger,
            &self.config.general.log_in_term,
        );
        logger.close();
        logger_handler.close();
    }
}
