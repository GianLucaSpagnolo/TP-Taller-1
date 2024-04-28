pub struct _VariableHeaderProtocolName {
    length_msb: u8,
    length_lsb: u8,
    name: u32,
}

pub struct _VariableHeaderKeepAlive {
    msb: u8,
    lsb: u8,
}

pub enum _VariableHeaderProperty {
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

pub struct _VariableHeaderProperties {
    length: usize,
    properties: Vec<_VariableHeaderProperty>,
}

pub struct _ConnectVariableHeader {
    protocol_name: _VariableHeaderProtocolName,
    protocol_version: u8,
    connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    keep_alive: _VariableHeaderKeepAlive,
    properties: _VariableHeaderProperties,
}

impl _ConnectVariableHeader {
    pub fn _lenght(&self) -> usize {
        todo!()
    }

    pub fn _new() -> Self {
        todo!()
    }
}
