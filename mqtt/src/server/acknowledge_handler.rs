use std::{io::Error, net::TcpStream};

use crate::{
    common::{flags::flags_handler, reason_codes::ReasonCode}, mqtt_packets::{
        packets::{
            connect::Connect, publish::Publish, subscribe::Subscribe, unsubscribe::Unsubscribe,
        },
        properties::{
            connack_properties::ConnackProperties, puback_properties::PubackProperties,
            suback_properties::SubackProperties, unsuback_properties::UnsubackProperties,
        },
    }
};

use super::mqtt_server::MqttServer;

pub fn determinate_connect_acknowledge(
    server: &mut MqttServer,
    connect: Connect,
    stream_connection: TcpStream,
) -> Result<ConnackProperties, Error> {
    // Si no recibe ninguna conexión en cierta cantidad de tiempo debe cortar la conexión (timer!)

    // Connect Flags:
    // - Will Retain: Si will flag == 0, will retain == 0.
    // Si will flag == 1, will retain puede ser 0 o 1. En caso de ser 1, el servidor debe almacenar el mensaje y enviarlo a los suscriptores en caso de que el cliente se desconecte
    // (si will retain == 0, debe enviarse como un normal message, si will retain == 1, debe enviarse como un Retained Message)
    // - Username y password flags determinan que hayan respectivos username y password en el payload del CONNECT
    // - Keep Alive: El tiempo en segundos que el cliente espera entre dos mensajes de control. Si el servidor no recibe un mensaje de control en ese tiempo, debe cerrar la conexion
    // Si keep alive != 0, el cliente debe enviar un PINGREQ packet al servidor en ese tiempo.
    // Si el servidor no recibe en x1.5 veces el tiempo de keep alive un MQTT Control Packet, debe cerrar la Network Connection como si haya fallado
    // Si el server envia un Server Keep Alive en el CONNACK packet, se debe usar ese valor

    // Se inicia la sesion de la conexion entre el cliente y el servidor.
    // El cliente y el servidor deben asociar el estado con el Client Identifier
    // A esto se lo llama Session State, y almacena las subscripciones
    // Se debe descartar la sesion unicamente cuando se cierra la conexion y el Session Expiry Interval pasó

    // let connack_properties = server.determinate_connack_properties(&connect);

    let mut connack_properties = ConnackProperties {
        connect_reason_code: determinate_reason_code(server, &connect),
        ..Default::default()
    };

    // Clean start: si es 1, el cliente y servidor deben descartar cualquier session state asociado con el Client Identifier. Session Present flag in connack = 0
    // Clean Start: si es 0, el cliente y servidor deben mantener el session state asociado con el Client Identifier.
    // En caso de que no exista dicha sesion, hay que crearla
    if flags_handler::get_connect_flag_clean_start(connect.properties.connect_flags) == 1 {
        server.register.clean_session(&connect.payload.client_id);
    }
    // - Will Flag: si es 1, un Will Message debe ser almacenado en el servidor y asociado a la sesion.
    // El will message esta compuesto de will properties, will topic y will payload fields del payload del CONNECT packet.
    // El will message debe ser publicado despues de que una network connection se cierra y la sesion expira, o el willdelay interval haya pasado
    // El will message debe ser borrado en caso de que el servidor reciba un DISCONNECT packet con reason code 0x00, o una nueva Network Connection con Clean Start = 1
    // con el mismo client identifier. Tambien debe ser borrado de la session state en caso de que ya haya sido publicado
    server.network.connections.insert(connect.payload.client_id.clone(), stream_connection);
    connack_properties.connect_acknowledge_flags = server.register.open_session(connect);

    Ok(connack_properties)
}

/// ### determinate_publish_acknowledge
///
/// Determina la respuesta a un paquete de publicación
///
/// ### Parametros
/// - `publish`: Paquete de publicación
///
/// ### Retorno
/// - `Result<PubackProperties, Error>`: Resultado de la operación
///     
pub fn determinate_publish_acknowledge(publish: Publish) -> Result<PubackProperties, Error> {
    let puback_properties = PubackProperties {
        packet_id: publish.properties.packet_identifier,
        puback_reason_code: ReasonCode::Success.get_id(),
        ..Default::default()
    };

    Ok(puback_properties)
}

/// ### determinate_subscribe_acknowledge
///
/// Determina la respuesta a un paquete de subscripción
///
/// ### Parametros
/// - `subscribe`: Paquete de subscripción
///
/// ### Retorno
/// - `Result<SubackProperties, Error>`: Resultado de la operación
///
pub fn determinate_subscribe_acknowledge(subscribe: Subscribe) -> Result<SubackProperties, Error> {
    let suback_properties = SubackProperties {
        packet_identifier: subscribe.properties.packet_identifier,
        reason_codes: vec![
            ReasonCode::Success.get_id(),
            ReasonCode::NotAuthorized.get_id(),
        ],
        ..Default::default()
    };

    Ok(suback_properties)
}

/// ### determinate_unsubscribe_acknowledge
///
/// Determina la respuesta a un paquete de desubscripción
///
/// ### Parametros
/// - `unsubscribe`: Paquete de desubscripción
///
/// ### Retorno
/// - `Result<UnsubackProperties, Error>`: Resultado de la operación
///
pub fn determinate_unsubscribe_acknowledge(
    unsubscribe: Unsubscribe,
) -> Result<UnsubackProperties, Error> {
    let unsuback_properties = UnsubackProperties {
        packet_identifier: unsubscribe.properties.packet_identifier,
        reason_codes: vec![ReasonCode::Success.get_id()],
        ..Default::default()
    };

    Ok(unsuback_properties)
}

/// ### determinate_reason_code
///
/// Determina el reason code de un paquete de conexión
///
/// ### Parametros
/// - `connect_packet`: Paquete de conexión
///
/// ### Retorno
/// - `u8`: Resultado de la operación
///
pub fn determinate_reason_code(server: &MqttServer, connect_packet: &Connect) -> u8 {
    // Si ya se recibió un CONNECT packet, se debe procesar como un Protocol Error (reason code 130) y cerrar la conexion.
    if server.connect_received {
        return ReasonCode::ProtocolError.get_id();
    }

    // Protocol Name: "MQTT" - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
    // Protocol Version: 5 - En caso de ser diferente, debe procesarlo como  Unsupported Protocol Version (reason code 132) y cerrar la conexion.
    if connect_packet.properties.protocol_name != *"MQTT"
        || connect_packet.properties.protocol_version != 5
    {
        return ReasonCode::UnsupportedProtocolVersion.get_id();
    }

    // Reserved: 0. En caso de recibir 1 debe devolver Malformed Packet (reason code 129) y cerrar la conexion
    if flags_handler::get_connect_flag_reserved(connect_packet.properties.connect_flags) != 0 {
        return ReasonCode::MalformedPacket.get_id();
    }

    // - Will QoS: 1. En caso de recibir 3 debe devolver QoS Not Supported (reason code 155) y cerrar la conexion
    if flags_handler::get_connect_flag_will_qos(connect_packet.properties.connect_flags) <= 1 {
        return ReasonCode::QoSNotSupported.get_id();
    }

    if !connect_packet
        .payload
        .client_id
        .chars()
        .all(|c| c.is_ascii_alphanumeric())
    {
        return ReasonCode::ClientIdentifierNotValid.get_id();
    }
    ReasonCode::Success.get_id()
}
