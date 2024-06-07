use std::fmt::Display;

use logger::logger_handler::Logger;

/// ## MqttActions
///
/// Trait que define las acciones de un cliente o servidor MQTT
///
/// ### Metodos
/// - `log_action`: Loguea la accion realizada
///
pub trait MqttActions<Role = Self> {
    /// ## log_action
    ///
    /// Loguea la accion realizada
    ///
    /// ### Parametros
    /// - `id`: identificador de la accion
    /// - `logger`: logger del servidor
    /// - `log_in_term`: loguear en terminal
    ///
    fn log_action(&self, id: &String, logger: &Logger, log_in_term: &bool)
    where
        Self: Display,
    {
        if *log_in_term {
            println!("{} - {}", id, self);
        }
        logger.log_event(&self.to_string(), id);
    }
}

/// ## add_topics_names
///
/// Agrega los nombres de los topics a un mensaje
///
/// ### Parametros
/// - `msg`: mensaje al que se le agregan los topics
/// - `topic`: nombre del topic
/// - `i`: contador de topics
///
pub fn add_topics_names(msg: &mut String, topic: &String, i: &mut usize) {
    if *i == 0 {
        *msg = msg.to_string() + topic;
    } else {
        *msg = msg.to_string() + " - " + topic;
    }
    *i += 1;
}
