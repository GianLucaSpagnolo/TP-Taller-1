use std::net::TcpStream;

use crate::control_packets::mqtt_connect::connect::*;

fn id_generator() -> String {
    //To Do
    "hola123".to_string()
}

pub fn client_connect(address: &str) -> std::io::Result<()> {
    let id = id_generator();
    let mut socket = TcpStream::connect(address)?;

    let connect_flags = create_connect_flags(0, 0, 1, 1, 1, 0, 0);
    let keep_alive: u16 = 0;

    // Deberia leerse de un archivo de configuracion
    let connect_properties = ConnectProperties {
        session_expiry_interval: 500,
        authentication_method: "password".to_string(),
        authentication_data: 1,
        request_problem_information: 0,
        request_response_information: 1,
        receive_maximum: 10,
        topic_alias_maximum: 0,
        user_property_key: "user".to_string(),
        user_property_value: "property".to_string(),
        maximum_packet_size: 100,
    };
    let connect_packet = Connect::new(id, connect_flags, keep_alive, connect_properties);

    match connect_packet.write_to(&mut socket) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }

    /* let connack_packet_as_bytes;

    match socket.read(connack_packet_as_bytes) {
        Ok(_) => Ok(println!("Connect complete")),
        Err(e) => Err(e),
    } */
}
