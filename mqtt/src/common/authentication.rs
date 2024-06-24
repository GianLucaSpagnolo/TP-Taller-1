pub fn serialize_username_password(username: &String, password: &String) -> Vec<u8> {
    let mut bytes = Vec::new();

    let user_len = username.len() as u16;
    bytes.extend_from_slice(&user_len.to_be_bytes());
    bytes.extend_from_slice(username.as_bytes());
    let pass_len = password.len() as u16;
    bytes.extend_from_slice(&pass_len.to_be_bytes());
    bytes.extend_from_slice(password.as_bytes());

    bytes
}

pub fn deserialize_username_password(buffer: Vec<u8>) -> (String, String) {
    let mut buffer = buffer.as_slice();

    let user_len = u16::from_be_bytes([buffer[0], buffer[1]]) as usize;
    buffer = &buffer[2..];
    let username = String::from_utf8(buffer[..user_len].to_vec()).unwrap();
    buffer = &buffer[user_len..];

    let pass_len = u16::from_be_bytes([buffer[0], buffer[1]]) as usize;
    buffer = &buffer[2..];
    let password = String::from_utf8(buffer[..pass_len].to_vec()).unwrap();

    (username, password)
}
