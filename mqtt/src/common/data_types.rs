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

    pub fn read_two_byte_integer(stream: &mut dyn Read) -> Result<u16, Error> {
        let mut read_buff = [0u8; 2];
        stream.read_exact(&mut read_buff)?;
        Ok(u16::from_be_bytes(read_buff))
    }

    #[allow(dead_code)]
    pub fn read_four_byte_integer(stream: &mut dyn Read) -> Result<u32, Error> {
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

    pub fn variable_byte_integer_encode(bytes: &mut Vec<u8>, value: u32) {
        if value == 0 {
            bytes.push(0);
            return;
        }

        let mut value = value;

        while value > 0 {
            let mut byte = (value % 128) as u8;
            value /= 128;

            // if there are more data to encode, set the top bit of this byte
            if value > 0 {
                byte |= 0x80;
            }
            bytes.push(byte);
        }
    }

    pub fn variable_byte_integer_decode(stream: &mut dyn Read) -> Result<u32, Error> {
        let mut multiplier = 1;
        let mut value = 0;

        loop {
            let byte = read_byte(stream)?;

            value += (byte & 0x7F) as u32 * multiplier;
            if multiplier > 128 * 128 * 128 {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Malformed Variable Byte Integer",
                ));
            }

            if byte & 0x80 == 0 {
                break;
            }
            multiplier *= 128;
        }

        Ok(value)
    }

    pub fn variable_byte_integer_length(value: u32) -> u32 {
        if value == 0 {
            return 1;
        }

        let mut value = value;
        let mut len = 0;

        while value > 0 {
            value /= 128;
            len += 1;
        }

        len
    }
}
