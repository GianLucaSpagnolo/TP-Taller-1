
/// Connect Reason Code
/// Byte 2 in the Variable Header is the Connect Reason Code.
/// 0 - 0x00 - Success
/// The Connection is accepted.
/// 128 - 0x80 - Unspecified error
/// The Server does not wish to reveal the reason for the failure, or none of the other Reason Codes apply.
/// 129 - 0x81 - Malformed Packet 
/// Data within the CONNECT packet could not be correctly parsed.
/// 130 - 0x82 - Protocol Error 
/// Data in the CONNECT packet does not conform to this specification.
/// 131 - 0x83 - Implementation specific error 
/// The CONNECT is valid but is not accepted by this Server.
/// 132 - 0x84 - Unsupported Protocol Version 
/// The Server does not support the version of the MQTT protocol requested by the Client.
/// 133 - 0x85 - Client Identifier not valid 
/// The Client Identifier is a valid string but is not allowed by the Server.
/// 134 - 0x86 - Bad User Name or Password 
/// The Server does not accept the User Name or Password specified by the Client
/// 135 - 0x87 - Not authorized 
/// The Client is not authorized to connect.
/// 136 - 0x88 - Server unavailable 
/// The MQTT Server is not available.
/// 137 - 0x89 - Server busy 
/// The Server is busy. Try again later.
/// 138 - 0x8A - Banned 
/// This Client has been banned by administrative action. Contact the server administrator.
/// 140 - 0x8C - Bad authentication method 
/// The authentication method is not supported or does not match the authentication method currently in use.
/// 144 - 0x90 - Topic Name invalid 
/// The Will Topic Name is not malformed, but is not accepted by this Server.
/// 149 - 0x95 - Packet too large 
/// The CONNECT packet exceeded the maximum permissible size.
/// 151 - 0x97 - Quota exceeded 
/// An implementation or administrative imposed limit has been exceeded.
/// 153 - 0x99 - Payload format invalid 
/// The Will Payload does not match the specified Payload Format Indicator.
/// 154 - 0x9A - Retain not supported 
/// The Server does not support retained messages, and Will Retain was set to 1.
/// 155 - 0x9B - QoS not supported 
/// The Server does not support the QoS set in Will QoS.
/// 156 - 0x9C - Use another server 
/// The Client should temporarily use another server.
/// 157 - 0x9D - Server moved 
/// The Client should permanently use another server.
/// 159 - 0x9F - Connection rate exceeded 
/// The connection rate limit has been exceeded. 


pub enum ConnectReasonMode{
    Success,
    _UnspecifiedError,
    _MalformedPacket,
    _ProtocolError,
    _ImplementationSpecificError,
    _UnsupportedProtocolVersion,
    _ClientIdentifierNotValid,
    _BadUserNameOrPassword,
    _NotAuthorized,
    _ServerUnavailable,
    _ServerBusy,
    _Banned,
    _BadAuthenticationMethod,
    _TopicNameInvalid,
    _PacketTooLarge,
    _QuotaExceeded,
    _PayloadFormatInvalid,
    _RetainNotSupported,
    _QoSNotSupported,
    _UseAnotherServer,
    _ServerMoved,
    _ConnectionRateExceeded,
}

impl ConnectReasonMode{
    pub fn get_id(&self) -> u8 {
        match *self{
            ConnectReasonMode::Success => 0,
            ConnectReasonMode::_UnspecifiedError => 128,
            ConnectReasonMode::_MalformedPacket => 129,
            ConnectReasonMode::_ProtocolError => 130,
            ConnectReasonMode::_ImplementationSpecificError => 131,
            ConnectReasonMode::_UnsupportedProtocolVersion => 132,
            ConnectReasonMode::_ClientIdentifierNotValid => 133,
            ConnectReasonMode::_BadUserNameOrPassword => 134,
            ConnectReasonMode::_NotAuthorized => 135,
            ConnectReasonMode::_ServerUnavailable => 136,
            ConnectReasonMode::_ServerBusy => 137,
            ConnectReasonMode::_Banned => 138,
            ConnectReasonMode::_BadAuthenticationMethod => 140,
            ConnectReasonMode::_TopicNameInvalid => 144,
            ConnectReasonMode::_PacketTooLarge => 149,
            ConnectReasonMode::_QuotaExceeded => 151,
            ConnectReasonMode::_PayloadFormatInvalid => 153,
            ConnectReasonMode::_RetainNotSupported => 154,
            ConnectReasonMode::_QoSNotSupported => 155,
            ConnectReasonMode::_UseAnotherServer => 156,
            ConnectReasonMode::_ServerMoved => 157,
            ConnectReasonMode::_ConnectionRateExceeded => 159,
        }
    }
}