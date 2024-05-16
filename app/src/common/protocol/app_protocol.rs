// protocolo de la app, usado por todos los clientes y el servidor

/// El protocolo traduce los paquetes de mqtt a comandos
/// entendibles por la app
///
/// para el connect:
///     * el mqtt recibe los bytes del cliente          -- OK
///          * los empaqueta y devuelve al protocolo    -- OK
///     * el protocolo recibe el paquete                -- OK
///          * traduce el paquete a acciones de la app  -- OK
///     * el protocolo traduce los paquetes a acciones de alto nivel
///       para el logger.
///

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
/// ## CONNECT FLAGS
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
/// ## Properties
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
/// # PAYLOAD
/// The Payload of the CONNECT packet contains one or more length-prefixed fields, whose presence is determined by the flags in the Variable Header.
/// The Payload contains one or more encoded fields. They specify a unique Client identifier for the Client, a Will Topic, Will Payload, User Name and
/// Password. All but the Client identifier can be omitted and their presence is determined based on flags in the Variable Header.
///
/// These fields, if present, MUST appear in the order:
/// Client Identifier: UTF-8 Encoded String (Obligatorio)
/// Will Properties:
///  - Property Length
///  - 24(0x18) - Will Delay Interval
///  - 1(0x01) - Payload Format Indicator
///  - 2(0x02) - Message Expiry Interval
///  - 3(0x03) - Content Type
///  - 8(0x08) - Response Topic
///  - 9(0x09) - Correlation Data
///  - 38(0x26) - User Property
/// Will Topic (Connect Flag - Will Flag = 1)
/// Will Payload (Connect Flag - Will Flag = 1)
/// Username (Connect Flag - Username = 1)
/// Password (Connect Flag - Password = 1)
///

/// para el connect:
///     * el mqtt recibe los bytes del cliente
///          * los empaqueta y devuelve al protocolo -- OK
///     * el protocolo recibe el paquete
///          * traduce el paquete a acciones de la app
///     * el protocolo traduce los paquetes a acciones de alto nivel
///       para el logger.
///
///

pub enum ProtocolActions {
    TryConnect, // guardara el exit code
    PackageError,
}
/*
pub fn server_bind_address(address: &String) -> Result<TcpListener, Error> {
    if !address.contains(':') {
        return Err(Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid ip - port format, must be: ip:port",
        ));
    }
    //mqtt::server::server_run_bind(address)
}
/
// usada por el servidor para recibir los paquetes
// del cliente
// el protocolo recibe el paquete, lo procesa y traduce el
// paquete a una accion que el servidor de la app comprenda.
pub fn receive_package(stream: &mut TcpStream, server: &mut Server) -> ProtocolActions {
    match server.process_packet(stream) {
        Ok(_) => todo!(),
        Err(_) => todo!(),
    }
}
*/
// translations
// segun el tipo de paquete, lo traduce a un mensaje
// entendible por el logger
pub fn translate_received_package(package_type: &ProtocolActions) -> String {
    match package_type {
        ProtocolActions::TryConnect => translate_connect_package(),
        ProtocolActions::PackageError => String::from("Unknow package received"),
    }
}

fn translate_connect_package() -> String {
    String::from("Client try to connect")
}

/*
// usada por el servidor para recibir los paquetes
// del cliente
// el protocolo recibe el paquete, lo procesa y traduce el
// paquete a una accion que el servidor de la app comprenda.
pub fn receive_package(stream: &mut TcpStream) -> ServerActions {
    // averiguo el tipo de paquete:
    let fixed_header = match pack_header_bytes(stream) {
        Some(header_type) => header_type,
        None => return None,
    };

    match get_package_type(&fixed_header) {
        Some(HeaderType::ConnectType) => {
            match get_package(stream, fixed_header, HeaderType::ConnectType) {
                Ok(pack) => match pack {
                    PackagedPackage::ConnectPackage(_pack) =>
                    // el servidor de la app debera poder
                    // ejecutar el connack, para esto,
                    // tanto el enum del server MQTT, como el
                    // enum del protocolo, deben de tener lo necesario
                    // para poder reconstruir los paquetes
                    {
                        Some(ServerActions::TryConnect)
                    }
                },
                Err(..) => None,
            }
        }
        None => None,
    }

    // le devuelve el paquete al servidor
    // el servidor lo pasa al logger
    // el logger le pide traduccion al protocolo
}
*/
