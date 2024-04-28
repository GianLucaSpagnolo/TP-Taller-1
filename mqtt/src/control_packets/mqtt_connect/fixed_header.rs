pub struct ConnectFixedHeader {
    pub packet_type_and_flags: u8,
    pub remaining_length: usize, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl ConnectFixedHeader {
    pub fn new(type_and_flags: u8, remaining_len: usize) -> Self{
        ConnectFixedHeader {
            packet_type_and_flags: type_and_flags,
            remaining_length: remaining_len
        }
    }
}