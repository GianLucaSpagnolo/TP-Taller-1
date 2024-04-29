pub struct ConnackFixedHeader {
    pub packet_type_and_flags: u8,
    pub remaining_length: u8,
}

impl ConnackFixedHeader {
    pub fn new(packet_type_and_flags: u8, remaining_length: u8) -> ConnackFixedHeader {
        ConnackFixedHeader {
            packet_type_and_flags,
            remaining_length,
        }
    }
}
