/// ## Suback packet
/// 
/// The SUBACK Packet is sent by the Server to the Client to confirm receipt and processing of a SUBSCRIBE Packet.
/// 
/// A SUBACK packet contais a list of reason codes, that specify the maximun QoS level that was granted or the error
/// which was found for each Subscription that was requested by the SUBSCRIBE.
/// 
/// ### Fixed Header
/// 
/// PRIMER BYTE
/// 4 bits mas significativos: MQTT Control Packet type
/// 1001: SUBACK
/// 
/// 4 bits menos significativos: Flags
/// 0000: Reserved
/// 
/// SEGUNDO BYTE
/// 
/// Remaining Length
/// This is the length of Variable Header plus the length of the Payload, encoded as a Variable Byte Integer.
/// 
/// ### Variable Header
/// 
/// The Variable Header of the SUBACK packet contains the following fields in the order: Packet Identifier from
/// the SUBSCRIBE packet that is being acknowledged, and Properties.
/// 
/// #### Packet Identifier
/// 
/// The Packet Identifier field contains the Packet Identifier from the SUBSCRIBE packet that is being acknowledged.
/// 
/// BYTE 1: Most Significant Byte of the Packet Identifier (MSB)
/// BYTE 2: Least Significant Byte of the Packet Identifier (LSB)
/// 
/// #### Properties
/// 
/// 1. Property Length
/// BYTE 3: Property Length (Variable Byte Integer)
/// 
/// 2. Reason String
/// ID: 31 (0x1F)
/// BYTE 4: Reason String (UTF-8 Encoded String)
/// 
/// 3. User Property
/// ID: 38 (0x26)
/// Byte n: User Property (UTF-8 String Pair)
/// 
/// ### Payload
/// 
/// The Payload of the SUBACK packet contains a list of Reason Codes. Each Reason Code corresponds to a Topic Filter
/// in the SUBSCRIBE packet that is being acknowledged.
/// The order of the Reason Codes in the SUBACK packet MUST match the order of Topic Filters in the SUBSCRIBE packet.
/// 
/// 

pub struct Suback {
    pub properties: _SubackProperties,
}