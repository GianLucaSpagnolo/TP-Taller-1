// Deserialize a Will Message payload
//
// The payload is composed of the message length and the message itself
// The message length is a 2 byte field
// The message is a UTF-8 string
//
pub fn deserialize_will_message_payload(payload: Vec<u8>) -> String {
    let message_len = u16::from_be_bytes([payload[0], payload[1]]) as usize;
    String::from_utf8(payload[2..message_len + 2].to_vec()).unwrap()
}

// Create a Will Message payload
//
// The payload is composed of the message length and the message itself
// The message length is a 2 byte field
// The message is a UTF-8 string
//
pub fn serialize_will_message_payload(message: String) -> Vec<u8> {
    let mut payload: Vec<u8> = Vec::new();
    let message_len = message.len() as u16;
    payload.extend_from_slice(&message_len.to_be_bytes());
    payload.extend_from_slice(message.as_bytes());

    payload
}
