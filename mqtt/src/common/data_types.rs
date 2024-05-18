pub mod data_representation {
    use std::{
        io::{Error, Read},
        mem::size_of,
        string::FromUtf8Error,
    };

    pub fn read_byte(stream: &mut dyn Read) -> Result<u8, Error> {
        let mut read_buff = [0u8; 1];
        stream.read_exact(&mut read_buff)?;
        Ok(u8::from_be_bytes(read_buff))
    }

    pub fn read_byte_buffer(buffer: &mut [u8]) -> Result<u8, Error> {
        let mut read_buff = [0u8; 1];

        //stream.read_exact(&mut read_buff)?;
        let mut handle = buffer.take(1);
        handle.read(&mut read_buff)?;
        Ok(u8::from_be_bytes(read_buff))
    }

    pub fn read_two_byte_integer(stream: &mut dyn Read) -> Result<u16, Error> {
        let mut read_buff = [0u8; 2];
        
        //stream.read_exact(&mut read_buff)?;
        let mut handle = stream.take(1);
        handle.read(&mut read_buff)?;
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

    pub fn four_byte_integer_from_be_bytes(buff: &[u8], buff_size: &mut usize) -> u32 {
        let mut local_buff: [u8; 4] = [0; 4];
        local_buff.copy_from_slice(&buff[*buff_size..*buff_size + 4]);
        *buff_size += size_of::<u32>();
        u32::from_be_bytes(local_buff)
    }

    pub fn two_byte_integer_from_be_bytes(buff: &[u8], buff_size: &mut usize) -> u16 {
        let mut local_buff: [u8; 2] = [0; 2];
        local_buff.copy_from_slice(&buff[*buff_size..*buff_size + 2]);
        *buff_size += size_of::<u16>();
        u16::from_be_bytes(local_buff)
    }

    pub fn byte_integer_from_be_bytes(buff: &[u8], buff_size: &mut usize) -> u8 {
        let value = buff[*buff_size];
        *buff_size += size_of::<u8>();
        value
    }

    pub fn utf8_string_from_be_bytes(
        buff: &[u8],
        length: u16,
        buff_size: &mut usize,
    ) -> Result<String, FromUtf8Error> {
        let mut local_buff: Vec<u8> = vec![0; length as usize];
        local_buff.copy_from_slice(&buff[*buff_size..*buff_size + length as usize]);
        *buff_size += length as usize;
        String::from_utf8(local_buff)
    }

    // ---------------------------
    pub fn read_two_byte_integer_buffer(buffer: &mut [u8]) -> Result<u16, Error> {
        let mut properties_len_1 = match buffer.get(0) {
            Some(r) => r,
            None => {
                eprintln!("Error al crear variable header properties desde un header");
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
            },
        };

        let mut properties_len_2 = match buffer.get(1) {
            Some(r) => r,
            None => {
                eprintln!("Error al crear variable header properties desde un header");
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Error al crear varaible header properties desde un header (vh properties"));
            },
        };

        const SIZEOFU16 :u16 = 2;
        let mut properties_len :u16 = (((properties_len_1 << 8) & properties_len_2)).into();
        Ok(properties_len)
    }

    pub fn read_utf8_encoded_string_buffer(stream: &mut [u8], length: u16) -> Result<String, Error> {
        let mut read_buff = vec![0u8; length as usize];
        
        //stream.read_exact(&mut read_buff)?;
        let mut handle = stream.take(1);
        handle.read(&mut read_buff)?;
        
        match String::from_utf8(read_buff) {
            Ok(utf8_string) => Ok(utf8_string),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e)),
        }
    }
}
