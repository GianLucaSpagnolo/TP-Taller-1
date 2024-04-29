use std::net::TcpStream;

use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_connect::variable_header::*;

fn id_generator() -> String {
    //To Do
    "abc123".to_string()
}

pub fn client_connect(address: &str) -> std::io::Result<()> {
    let id = id_generator();
    let mut socket = TcpStream::connect(address)?;

    let connect_flags = create_connect_flags(0, 0, 1, 1, 1, 0, 0);
    let keep_alive: u16 = 0;
    // La inicializacion de las propiedades deben estar en connect.rs (add_variable_header_properties)
    // Faltan inicializar variables de la instancia del cliente (ejemplo: autentificacion, etc.)
    let mut properties = VariableHeaderProperties::new();
    properties.add_property_session_expiry_interval(500);
    properties.add_property_authentication_method("passwrod".to_string());
    properties.add_property_authentication_data(1);
    properties.add_property_request_problem_information(0);
    properties.add_property_request_response_information(1);
    properties.add_property_receive_maximum(10);
    properties.add_property_topic_alias_maximum(0);
    //properties.add_property_user_property("user".to_string(), "property".to_string());
    properties.add_property_maximum_packet_size(100);

    let connect_packet = Connect::new(id, connect_flags, keep_alive, properties);

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
