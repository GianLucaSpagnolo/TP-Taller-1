#[derive(Clone, Debug)]

/// ## TopicFilter
/// 
/// Estructura que representa un filtro de topicos
/// 
/// ### Atributos
/// - `topic_filter`: Filtro de topicos
/// - `subscription_options`: Opciones de suscripción
/// 
/// ### Consideraciones
/// - El filtro de topicos debe ser un string
/// - Las opciones de suscripción deben ser un byte
/// - Cada Topic Filter debe ser seguido por el Subscriptions Options Byte
/// 
pub struct TopicFilter {
    pub topic_filter: String,
    pub subscription_options: u8,
}
