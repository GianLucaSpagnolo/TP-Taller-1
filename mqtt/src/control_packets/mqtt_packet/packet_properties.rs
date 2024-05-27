use std::io::{Error, Read};

use super::variable_header_properties::VariableHeaderProperties;

/// ## PacketProperties
/// 
/// Trait que define las propiedades de un paquete
/// 
/// ### Metodos
/// - `size_of`: Devuelve la cantidad de bytes que ocupa el paquete
/// - `as_variable_header_properties`: Devuelve las propiedades del paquete como un objeto VariableHeaderProperties
/// - `as_bytes`: Devuelve las propiedades del paquete como un vector de bytes
/// - `read_from`: Lee las propiedades del paquete desde un stream
/// 
pub trait PacketProperties<Properties = Self> {

    /// ## size_of
    /// 
    /// Devuelve la cantidad de bytes que ocupa el paquete 
    /// 
    fn size_of(&self) -> u32;

    /// ## as_variable_header_properties
    /// 
    /// Devuelve las propiedades del paquete como un objeto VariableHeaderProperties
    /// 
    /// ### Retorno
    /// - `Result<VariableHeaderProperties, Error>`:
    ///    - Ok: propiedades del paquete
    ///    - Err: error al convertir las propiedades del paquete (std::io::Error)
    /// 
    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error>;

    /// ## as_bytes
    /// 
    /// Devuelve las propiedades del paquete como un vector de bytes
    /// 
    /// ### Retorno
    /// - `Result<Vec<u8>, Error>`:
    ///   - Ok: vector de bytes
    ///   - Err: error al convertir las propiedades del paquete (std::io::Error)
    /// 
    fn as_bytes(&self) -> Result<Vec<u8>, Error>;

    /// ## read_from
    /// 
    /// Lee las propiedades del paquete desde un stream
    /// 
    /// ### Parametros
    /// - `stream`: stream de bytes
    /// 
    /// ### Retorno
    /// - `Result<Properties, Error>`:
    ///   - Ok: propiedades del paquete
    ///   - Err: error al leer las propiedades del paquete (std::io::Error)
    /// 
    fn read_from(stream: &mut dyn Read) -> Result<Self, Error>
    where
        Self: Sized;
}
