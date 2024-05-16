use std::fmt;

use app::logger::LoggerHandler;

// implementar el encapsulamiento de client_id en cada uno
pub enum MqttActions {
    ServerConnection(String),
    ClientConnection(String, u8),
    ClientReceive(String, String),
    DisconnectClient,
    MessageReceived,
    MessageSended,
    TryConnect, // guardara el exit code
    PackageError,
}

impl MqttActions {
    pub fn register_action(self, logger: &LoggerHandler) -> Self {
        match &self {
            MqttActions::ServerConnection(id) => {
                logger.log_event(&"Connection successful".to_string(), id, &",".to_string())
            }
            MqttActions::ClientConnection(_, _) => todo!(),
            MqttActions::ClientReceive(_, _) => todo!(),
            MqttActions::DisconnectClient => todo!(),
            MqttActions::MessageReceived => logger.log_event(
                &"Message received".to_string(),
                &"?".to_string(),
                &",".to_string(),
            ),
            MqttActions::MessageSended => todo!(),
            MqttActions::TryConnect => todo!(),
            MqttActions::PackageError => todo!(),
        }
        self
    }
}

impl fmt::Display for MqttActions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MqttActions::ServerConnection(id) => write!(f, "Conexion establecida con {}", id),
            MqttActions::ClientConnection(addrs, reason_code) => write!(
                f,
                "Conexion establecida con {} y reason code: {}",
                addrs, reason_code
            ),
            MqttActions::ClientReceive(id, msg) => write!(f, "Cliente '{}' recibio: {}", id, msg),
            MqttActions::TryConnect => write!(f, "Intentando conectar"),
            MqttActions::DisconnectClient => write!(f, "Desconectando cliente"),
            MqttActions::MessageReceived => write!(f, "Mensaje recibido"),
            MqttActions::MessageSended => write!(f, "Mensaje enviado"),
            MqttActions::PackageError => write!(f, "Error en el paquete"),
        }
    }
}
