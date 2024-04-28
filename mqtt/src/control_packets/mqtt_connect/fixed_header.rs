pub struct ConnectFixedHeader {
    packet_type_and_flags: u8,
    remaining_length: usize, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}
