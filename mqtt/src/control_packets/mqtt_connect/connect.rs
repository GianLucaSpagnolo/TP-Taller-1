use std::io::Error;
use std::io::Read;
use std::io::Write;

use crate::control_packets::mqtt_connect::fixed_header::*;
use crate::control_packets::mqtt_connect::variable_header::*;

use super::variable_header_properties::VariableHeaderProperties;
use crate::control_packets::mqtt_connect::connect_payload::*;

static PROTOCOL_NAME: &str = "MQTT";
static PROTOCOL_VERSION: u8 = 5;

pub struct Connect {
    pub fixed_header: ConnectFixedHeader,
    pub variable_header: ConnectVariableHeader,
    pub payload: ConnectPayload,
}

pub struct ConnectProperties {
    pub session_expiry_interval: u32,
    pub authentication_method: String,
    pub authentication_data: u16,
    pub request_problem_information: u8,
    pub request_response_information: u8,
    pub receive_maximum: u16,
    pub topic_alias_maximum: u16,
    pub user_property_key: String,
    pub user_property_value: String,
    pub maximum_packet_size: u32,
}

pub fn create_connect_flags(
    reserver: u8,
    clean_start: u8,
    will_flag: u8,
    will_qos: u8,
    will_retain: u8,
    password: u8,
    username: u8,
) -> u8 {
    let mut connect_flags: u8 = 0;
    connect_flags |= reserver;
    connect_flags |= clean_start << 1;
    connect_flags |= will_flag << 2;
    connect_flags |= will_qos << 3;
    connect_flags |= will_retain << 5;
    connect_flags |= password << 6;
    connect_flags |= username << 7;
    connect_flags
}

fn apply_mask_to_n_bits(flags: u8, shifts: u8, len: u8) -> u8 {
    let mask = (1 << len) - 1;
    (flags >> shifts) & mask
}

/// FLAG: RESERVED
pub fn get_flag_reserved(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 0, 1)
}

/// FLAG: CLEAN START
pub fn get_flag_clean_start(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 1, 1)
}

/// FLAG: WILL FLAG
pub fn get_flag_will_flag(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 2, 1)
}

/// FLAG: WILL QoS
pub fn get_flag_will_qos(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 3, 2)
}

/// FLAG: WILL RETAIN
pub fn get_flag_will_retain(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 5, 1)
}

/// FLAG: PASSWORD
pub fn get_flag_password(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 6, 1)
}

/// FLAG: USERNAME
pub fn get_flag_username(flags: u8) -> u8 {
    apply_mask_to_n_bits(flags, 7, 1)
}

/// # FIXED HEADER: 2 BYTES
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 0001: CONNECT
///
/// 4 bits menos significativos: Flags
/// 0000: Reserved
///
/// 00010000 CONNECT 16
///
/// SEGUNDO BYTE
/// Remaining Length
/// This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
///
/// # VARIABLE HEADER: Packet Identifier de 2 BYTES
///
/// CONNECT no necesita el Package Identifier
///
///
/// Ejemplo no normativo:
///
/// Protocol Name
/// byte 1 - Length MSB (0)
/// byte 2 - Length LSB (4)
/// byte 3 - ‘M’
/// byte 4 - ‘Q’
/// byte 5 - ‘T’
/// byte 6 - ‘T’
///
/// Protocol Version
/// Description
/// byte 7 - Version (5)
///
/// Connect Flags
/// byte 8
/// User Name Flag (1)
/// Password Flag (1)
/// Will Retain (0)
/// Will QoS (01)
/// Will Flag (1)
/// Clean Start(1)
/// Reserved (0)
///
/// Keep Alive
/// byte 9
/// Keep Alive MSB (0)
/// byte 10
/// Keep Alive LSB (10)
///
/// Properties
/// byte 11
/// Length (suma de todas las properties)
/// byte 12 en adelante:
/// PROPERTIES: Connect
/// 17 - 0x11 - Session Expiry Interval - Four Byte Integer
/// 21 - 0x15 - Authentication Method - UTF-8 Encoded String
/// 22 - 0x16 - Authentication Data - Binary Data
/// 23 - 0x17 - Request Problem Information - Byte
/// 25 - 0x19 - Request Response Information - Byte
/// 33 - 0x21 - Receive Maximum - Two Byte Integer
/// 34 - 0x22 - Topic Alias Maximum - Two Byte Integer
/// 38 - 0x26 - User Property - UTF-8 String Pair
/// 39 - 0x27 - Maximum Packet Size - Four Byte Integer
///
///
/// PAYLOAD
/// The Payload of the CONNECT packet contains one or more length-prefixed fields, whose presence is determined by the flags in the Variable Header.
/// The Payload contains one or more encoded fields. They specify a unique Client identifier for the Client, a Will Topic, Will Payload, User Name and
/// Password. All but the Client identifier can be omitted and their presence is determined based on flags in the Variable Header.
///
/// These fields, if present, MUST appear in the order:
/// Client Identifier: UTF-8 Encoded String
/// Will Properties:
///  - Property Length
///  - Will Delay Interval
///  - Payload Format Indicator
///  - Message Expiry Interval
///  - Content Type
///  - Response Topic
///  - Correlation Data
///  - User Property
/// Will Topic
/// Will Payload
/// Username
/// Password
///
impl Connect {
    pub fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header_type_and_flags = self.fixed_header.packet_type_and_flags.to_be_bytes();
        let fixed_header_length = self.fixed_header.remaining_length.to_be_bytes();
        stream.write_all(&fixed_header_type_and_flags)?;
        stream.write_all(&fixed_header_length)?;

        let variable_header_protocol_name_length =
            self.variable_header.protocol_name.length.to_be_bytes();
        let variable_header_protocol_name = self.variable_header.protocol_name.name.as_bytes();
        let variable_header_protocol_version = self.variable_header.protocol_version.to_be_bytes();
        let variable_header_connect_flags = self.variable_header.connect_flags.to_be_bytes();
        let variable_header_keep_alive = self.variable_header.keep_alive.to_be_bytes();
        let variable_header_properties = self.variable_header.properties.as_bytes();
        stream.write_all(&variable_header_protocol_name_length)?;
        stream.write_all(variable_header_protocol_name)?;
        stream.write_all(&variable_header_protocol_version)?;
        stream.write_all(&variable_header_connect_flags)?;
        stream.write_all(&variable_header_keep_alive)?;
        stream.write_all(&variable_header_properties)?;

        let payload_fields = self.payload.as_bytes();
        stream.write_all(&payload_fields)?;

        Ok(())
    }

    pub fn read_from(stream: &mut dyn Read) -> Result<Connect, Error> {
        let mut read_fixed_header_type = [0u8; 1];
        stream.read_exact(&mut read_fixed_header_type)?;
        let fixed_header_type = u8::from_be_bytes(read_fixed_header_type);

        let mut read_fixed_header_len = [0u8; 8];
        stream.read_exact(&mut read_fixed_header_len)?;
        let fixed_header_len = usize::from_be_bytes(read_fixed_header_len);

        let mut read_variable_header_protocol_name_length = [0u8; 2];
        stream.read_exact(&mut read_variable_header_protocol_name_length)?;
        let protocol_name_length = u16::from_be_bytes(read_variable_header_protocol_name_length);

        let mut read_variable_header_protocol_name = vec![0u8; protocol_name_length as usize];
        stream.read_exact(&mut read_variable_header_protocol_name)?;
        let protocol_name = String::from_utf8(read_variable_header_protocol_name).unwrap();

        let mut read_variable_header_protocol_version = [0u8; 1];
        stream.read_exact(&mut read_variable_header_protocol_version)?;
        let protocol_version = u8::from_be_bytes(read_variable_header_protocol_version);

        let mut read_variable_header_connect_flags = [0u8; 1];
        stream.read_exact(&mut read_variable_header_connect_flags)?;
        let connect_flags = u8::from_be_bytes(read_variable_header_connect_flags);

        let mut read_variable_header_keep_alive = [0u8; 2];
        stream.read_exact(&mut read_variable_header_keep_alive)?;
        let keep_alive = u16::from_be_bytes(read_variable_header_keep_alive);

        let mut read_variable_header_properties_length = [0u8; 8];
        stream.read_exact(&mut read_variable_header_properties_length)?;
        let properties_length = usize::from_be_bytes(read_variable_header_properties_length);

        let mut read_variable_header_properties = vec![0u8; properties_length];
        stream.read_exact(&mut read_variable_header_properties)?;
        let properties = VariableHeaderProperties::from_be_bytes(&read_variable_header_properties);

        let mut read_payload_lenght_bytes = [0u8; 8];
        stream.read_exact(&mut read_payload_lenght_bytes)?;
        let _payload_length = usize::from_be_bytes(read_payload_lenght_bytes);

        let mut read_payload_client_id_len = [0u8; 2];
        stream.read_exact(&mut read_payload_client_id_len)?;
        let payload_client_id_len = u16::from_be_bytes(read_payload_client_id_len);

        let mut read_payload_client_id = vec![0u8; payload_client_id_len as usize];
        stream.read_exact(&mut read_payload_client_id)?;
        let payload_client_id = String::from_utf8(read_payload_client_id).unwrap();

        let connect = Connect {
            fixed_header: ConnectFixedHeader::new(fixed_header_type, fixed_header_len),
            variable_header: ConnectVariableHeader::new(
                protocol_name_length,
                protocol_name,
                protocol_version,
                connect_flags,
                keep_alive,
                properties,
            ),
            payload: ConnectPayload::new(payload_client_id),
        };
        Ok(connect)
    }

    pub fn new(
        client_id: String,
        connect_flags: u8,
        keep_alive: u16,
        properties: ConnectProperties,
    ) -> Self {
        let name = PROTOCOL_NAME.to_string();

        // La inicializacion de las propiedades deben estar en connect.rs (add_variable_header_properties)
        // Faltan inicializar variables de la instancia del cliente (ejemplo: autentificacion, etc.)
        let mut prop = VariableHeaderProperties::new();

        prop.add_property_session_expiry_interval(properties.session_expiry_interval);
        prop.add_property_authentication_method(properties.authentication_method);
        prop.add_property_authentication_data(properties.authentication_data);
        prop.add_property_request_problem_information(properties.request_problem_information);
        prop.add_property_request_response_information(properties.request_response_information);
        prop.add_property_receive_maximum(properties.receive_maximum);
        prop.add_property_topic_alias_maximum(properties.topic_alias_maximum);
        prop.add_property_user_property(
            properties.user_property_key,
            properties.user_property_value,
        );
        prop.add_property_maximum_packet_size(properties.maximum_packet_size);

        let variable_header = ConnectVariableHeader::new(
            name.len() as u16,
            name,
            PROTOCOL_VERSION,
            connect_flags,
            keep_alive,
            prop,
        );
        let payload = ConnectPayload::new(client_id);
        let remaining_length = variable_header.length() + payload.length();
        let fixed_header = ConnectFixedHeader::new(16, remaining_length);

        Connect {
            fixed_header,
            variable_header,
            payload,
        }
    }
}
