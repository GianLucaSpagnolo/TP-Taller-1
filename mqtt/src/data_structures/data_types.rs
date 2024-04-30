pub mod data_types {
    use std::io::{Error, Read};

    pub fn read_byte(stream: &mut dyn Read) -> Result<u8, Error> {
        let mut read_buff = [0u8; 1];
        stream.read_exact(&mut read_buff)?;
        Ok(u8::from_be_bytes(read_buff))
    }

    pub fn read_two_byte_integer(stream: &mut dyn Read) -> Result<u16, Error> {
        let mut read_buff = [0u8; 2];
        stream.read_exact(&mut read_buff)?;
        Ok(u16::from_be_bytes(read_buff))
    }

    pub fn _read_four_byte_integer(stream: &mut dyn Read) -> Result<u32, Error> {
        let mut read_buff = [0u8; 4];
        stream.read_exact(&mut read_buff)?;
        Ok(u32::from_be_bytes(read_buff))
    }

    pub fn read_utf8_encoded_string(stream: &mut dyn Read, length: u16) -> Result<String, Error> {
        let mut read_buff = vec![0u8; length as usize];
        stream.read_exact(&mut read_buff)?;

        match String::from_utf8(read_buff) {
            Ok(utf8_string) => Ok(utf8_string),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}
