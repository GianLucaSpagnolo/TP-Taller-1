pub struct VariableHeaderProtocolName {
    pub length: u16,
    pub name: String,
}

pub struct VariableHeaderKeepAlive {
    msb: u8,
    lsb: u8,
}

pub enum VariableHeaderProperty {
    SessionExpiryInterval { id: u8, property: u32 }, // Four Byte Integer
    AuthenticationMethod { id: u8, property: String }, // UTF-8 Encoded String
    AuthenticationData { id: u8, property: u16 },    // Binary Data
    RequestProblemInformation { id: u8, property: u8 }, // Byte
    RequestResponseInformation { id: u8, property: u8 }, // Byte
    ReceiveMaximum { id: u8, property: u16 },        // Two Byte Integer
    TopicAliasMaximum { id: u8, property: u16 },     // Two Byte Integer
    UserProperty { id: u8, property: (String, String) }, // UTF-8 String Pair
    MaximumPacketSize { id: u8, property: u32 },     // Four Byte Integer
}

pub struct VariableHeaderProperties {
    length: usize,
    properties: Vec<VariableHeaderProperty>,
}

pub struct ConnectVariableHeader {
    pub protocol_name: VariableHeaderProtocolName,
    pub protocol_version: u8,
    connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    keep_alive: VariableHeaderKeepAlive,
    properties: VariableHeaderProperties,
}

impl ConnectVariableHeader {
    pub fn lenght(&self) -> usize {
        todo!()
    }

    pub fn new(protocol_name_length: u16, protocol_name: String, protocol_version: u8) -> Self {
        ConnectVariableHeader {
            protocol_name: VariableHeaderProtocolName {
                length: protocol_name_length,
                name: protocol_name,
            },
            protocol_version,
            connect_flags: 0,
            keep_alive: VariableHeaderKeepAlive { msb: 0, lsb: 60 },
            properties: VariableHeaderProperties {
                length: 0,
                properties: Vec::new(),
            },
        }
    }
}
