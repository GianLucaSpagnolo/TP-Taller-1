struct VariableHeaderProtocolName {
    length_msb: u8,
    length_lsb: u8,
    name: u32,
}

struct VariableHeaderKeepAlive {
    msb: u8,
    lsb: u8,
}

enum VariableHeaderProperty {
    SessionExpiryInterval { id: u8, property: u32 },            // Four Byte Integer
    AuthenticationMethod { id: u8, property: String },          // UTF-8 Encoded String
    AuthenticationData { id: u8, property: u16 },               // Binary Data
    RequestProblemInformation { id: u8, property: u8 },         // Byte
    RequestResponseInformation { id: u8, property: u8 },        // Byte
    ReceiveMaximum { id: u8, property: u16 },                   // Two Byte Integer
    TopicAliasMaximum { id: u8, property: u16 },                // Two Byte Integer
    UserProperty { id: u8, property: (String, String) },        // UTF-8 String Pair
    MaximumPacketSize { id: u8, property: u32 },                // Four Byte Integer
}

struct VariableHeaderProperties {
    length: usize,
    properties: Vec<VariableHeaderProperty>,
}

struct ConnectVariableHeader {
    protocol_name: VariableHeaderProtocolName,
    protocol_version: u8,
    connect_flags: u8, // Nombre de los bits: User Name Flag, Password Flag, Will Retain, Will QoS (2 bytes), Will Flag, Clean Start, Reserved
    keep_alive: VariableHeaderKeepAlive,
    properties: VariableHeaderProperties,
}
