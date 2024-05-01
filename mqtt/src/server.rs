use std::io::Error;
use std::net::TcpListener;
use std::net::TcpStream;

use crate::control_packets::mqtt_connack::connack::Connack;
use crate::control_packets::mqtt_connack::connack::ConnackProperties;
use crate::control_packets::mqtt_connect::connect::*;
use crate::control_packets::mqtt_packet::flags::flags_handler;
use crate::control_packets::mqtt_packet::reason_codes::ReasonMode;
// agregado para protocolo
use crate::control_packets::mqtt_packet::fixed_header::*;

pub fn server_run(address: &str) -> Result<(), Error> {
    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => return Err(e),
    };
    for client_stream in listener.incoming() {
        match client_stream {
            Ok(mut stream) => {
                handle_connection(&mut stream)?;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn determinate_reason_code(connect_packet: &Connect) -> u8 {
    if connect_packet.variable_header.protocol_name.name != *"MQTT"
        || connect_packet.variable_header.protocol_version != 5
    {
        return ReasonMode::UnsupportedProtocolVersion.get_id();
    }

    if flags_handler::get_connect_flag_reserved(connect_packet.variable_header.connect_flags) != 0 {
        return ReasonMode::MalformedPacket.get_id();
    }

    if flags_handler::get_connect_flag_will_qos(connect_packet.variable_header.connect_flags) != 1 {
        return ReasonMode::QoSNotSupported.get_id();
    }

    if !connect_packet
        .payload
        .fields
        .client_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric())
    {
        return ReasonMode::ClientIdentifierNotValid.get_id();
    }
    ReasonMode::Success.get_id()
}

// refactorizacion para el protocolo
// captura los bytes de la red, lee el tipo
pub fn pack_header_bytes(stream: &mut TcpStream) -> Option<PacketFixedHeader> {
    match PacketFixedHeader::read_from(stream) {
        Ok(header) => Some(header),
        Err(..) => None,
    }
}

// Devuelve los bytes empaquetados en la estructura
// correspondiente.
pub fn pack_connect_bytes(
    stream: &mut TcpStream,
    fixed_header: PacketFixedHeader,
) -> Option<Connect> {
    match fixed_header.get_package_type() {
        PackageType::Connect => {
            // empaqueta el connect
            match Connect::read_from_header(stream, fixed_header) {
                Ok(connect_packcage) => Some(connect_packcage),
                Err(..) => None,
            }
        }
        PackageType::Unknow => None,
    }
}

fn handle_connection(stream: &mut TcpStream) -> Result<(), Error> {
    let connect = match Connect::read_from(stream) {
        Ok(p) => {
            println!(
                "Connect packet received\n
            Fixed header type and flags: {}\n
            Fixed header remaining length: {}\n
            Variable header protocol name length: {}\n
            Variable header protocol name: {}\n
            Variable header protocol version: {}\n
            Variable header flags reserver: {:01b}
            Variable header flags clean_start: {:01b}
            Variable header flags will_flag: {:01b}
            Variable header flags will_qos: {:02b}
            Variable header flags will_retain: {:01b}
            Variable header flags password: {:01b}
            Variable header flags username: {:01b}\n
            Variable header keep alive: {}\n
            Variable header property length: {}\n
            Variable header properties: {:?}\n
            Payload client id: {}",
                p.fixed_header.packet_type,
                p.fixed_header.remaining_length,
                p.variable_header.protocol_name.length,
                p.variable_header.protocol_name.name,
                p.variable_header.protocol_version,
                flags_handler::get_connect_flag_reserved(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_clean_start(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_will_flag(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_will_qos(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_will_retain(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_password(p.variable_header.connect_flags),
                flags_handler::get_connect_flag_username(p.variable_header.connect_flags),
                p.variable_header.keep_alive,
                p.variable_header.properties.properties.len(),
                p.variable_header.properties.properties,
                p.payload.fields.client_id
            );
            p
        }
        Err(e) => return Err(e),
    };

    let connack_properties = ConnackProperties {
        session_expiry_interval: 500,
        assigned_client_identifier: "client".to_string(),
        server_keep_alive: 10,
        authentication_method: "password".to_string(),
        authentication_data: 1,
        response_information: "response".to_string(),
        server_reference: "reference".to_string(),
        reason_string: "reason".to_string(),
        receive_maximum: 10,
        topic_alias_maximum: 0,
        maximum_qos: 2,
        retain_available: 1,
        wildcard_subscription_available: 1,
        subscription_identifiers_available: 1,
        shared_subscription_available: 1,
        user_property: ("key".to_string(), "value".to_string()),
        maximum_packet_size: 100,
    };

    let connect_acknowledge_flags = flags_handler::create_connect_acknowledge_flags(1);

    let connack_reason_code = determinate_reason_code(&connect);

    let connack_packet = Connack::new(
        connack_reason_code,
        connect_acknowledge_flags,
        connack_properties,
    )?;

    match connack_packet.write_to(stream) {
        Ok(_) => {}
        Err(e) => return Err(e),
    };

    Ok(())
}
