use std::io::{Error, Read};

pub trait PacketVariableHeader<Properties = Self> {
    fn length(&self) -> u16;
    fn as_bytes(&self) -> Vec<u8>;
    fn read_from(stream: &mut dyn Read) -> Result<Self, Error>
    where
        Self: Sized;
    fn new(props: &Properties) -> Result<Self, Error>
    where
        Self: Sized;
}
