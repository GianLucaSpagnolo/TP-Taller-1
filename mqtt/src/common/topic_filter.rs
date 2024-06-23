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

impl TopicFilter {
    /// ### as_bytes
    ///
    /// Convierte el filtro de topicos a un vector de bytes
    ///
    /// ### Retorno
    /// - `Vec<u8>`: Vector de bytes
    ///
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let len = self.topic_filter.len() as u8 + 1;
        bytes.extend_from_slice(&len.to_be_bytes());
        bytes.extend_from_slice(self.topic_filter.as_bytes());
        bytes.push(self.subscription_options);

        bytes
    }

    /// ### from_be_bytes
    ///
    /// Convierte un vector de bytes en un filtro de topicos
    ///
    /// ### Parametros
    /// - `bytes`: Vector de bytes
    ///
    /// ### Retorno
    /// - `TopicFilter`: Filtro de topicos creado
    ///
    pub fn from_be_bytes(bytes: Vec<u8>) -> Self {
        let mut index = 0;

        let topic_filter_len = bytes[index];
        index += 1;
        let topic_filter = String::from_utf8(bytes[index..topic_filter_len as usize].to_vec()).unwrap();
        index += topic_filter.len();
        let subscription_options = bytes[index];

        TopicFilter {
            topic_filter,
            subscription_options,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize() {
        let topic_filter = TopicFilter {
            topic_filter: "topic".to_string(),
            subscription_options: 0,
        };

        let bytes = topic_filter.as_bytes();
        let deserialized = TopicFilter::from_be_bytes(bytes);

        assert_eq!(topic_filter.topic_filter, deserialized.topic_filter);
        assert_eq!(topic_filter.subscription_options, deserialized.subscription_options);
    }
}
