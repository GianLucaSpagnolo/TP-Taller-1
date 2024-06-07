/// ## MqttClientMessage
///
/// Estructura que representa un mensaje recibido por el cliente MQTT.
///
/// ### Atributos
/// - topic: Tópico del mensaje.
/// - data: Datos del mensaje.
///
pub struct MqttClientMessage {
    pub topic: String,
    pub data: Vec<u8>,
}
