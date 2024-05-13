use std::io::{Error, Read};

use super::variable_header_properties::VariableHeaderProperties;

pub trait PacketProperties<Properties = Self> {
    /// Devuelve la cantidad de propiedades opcionales que tiene el paquete
    fn variable_props_size(&self) -> u16;

    /// Devuelve la cantidad de bytes que ocupa el paquete
    fn size_of(&self) -> u16;

    /// Devuelve las propiedades del paquete como un objeto VariableHeaderProperties
    fn as_variable_header_properties(&self) -> Result<VariableHeaderProperties, Error>;

    /// Devuelve las propiedades del paquete como un vector de bytes
    fn as_bytes(&self) -> Result<Vec<u8>, Error>;

    /// Lee las propiedades del paquete desde un stream
    fn read_from(stream: &mut dyn Read) -> Result<Self, Error>
    where
        Self: Sized;
}
