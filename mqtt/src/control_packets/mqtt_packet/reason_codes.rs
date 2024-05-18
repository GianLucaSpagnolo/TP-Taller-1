use std::fmt::Display;



/// Connect Reason Code
/// Byte 2 in the Variable Header is the Connect Reason Code.
/// 0 - 0x00 - Success
/// The Connection is accepted.
/// 16 - 0x10 - No matching subscribers
/// No matching subscribers. The Client or Server will not forward the PUBLISH packet.
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
/// 145 - 0x91 - Packet Identifier in use
/// The Packet Identifier is already in use. This will only ever be returned for a CONNACK or PUBACK packet.
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

pub enum ReasonCode {
    Success,
    _NormalDisconnection,
    _DisconnectWithWillMessage,
    _NoMatchingSubscribers,
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
    _ServerShuttingDown,
    _BadAuthenticationMethod,
    _KeepAliveTimeout,
    _SessionTakenOver,
    _TopicFilterInvalid,
    _TopicNameInvalid,
    _PacketIdentifierInUse,
    _ReceiveMaximumExceeded,
    _TopicAliasInvalid,
    _PacketTooLarge,
    _MessageRateTooHigh,
    _QuotaExceeded,
    _AdministrativeAction,
    _PayloadFormatInvalid,
    _RetainNotSupported,
    _QoSNotSupported,
    _UseAnotherServer,
    _ServerMoved,
    _SharedSubscriptionsNotSupported,
    _ConnectionRateExceeded,
    _MaximumConnectTime,
    _SubscriptionIdentifiersNotSupported,
    _WildcardSubscriptionsNotSupported,
}

impl ReasonCode {
    pub fn get_id(&self) -> u8 {
        match *self {
            ReasonCode::Success => 0,
            ReasonCode::_NormalDisconnection => 0, // DISCONNECT
            ReasonCode::_DisconnectWithWillMessage => 4, // DISCONNECT
            ReasonCode::_NoMatchingSubscribers => 16, // PUBACK
            ReasonCode::_UnspecifiedError => 128,  // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_MalformedPacket => 129,   // DISCONNECT
            ReasonCode::_ProtocolError => 130,     // DISCONNECT
            ReasonCode::_ImplementationSpecificError => 131, // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_UnsupportedProtocolVersion => 132,
            ReasonCode::_ClientIdentifierNotValid => 133,
            ReasonCode::_BadUserNameOrPassword => 134,
            ReasonCode::_NotAuthorized => 135, // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_ServerUnavailable => 136,
            ReasonCode::_ServerBusy => 137, // DISCONNECT
            ReasonCode::_Banned => 138,
            ReasonCode::_ServerShuttingDown => 139, // DISCONNECT
            ReasonCode::_BadAuthenticationMethod => 140,
            ReasonCode::_KeepAliveTimeout => 141, // DISCONNECT
            ReasonCode::_SessionTakenOver => 142, // DISCONNECT
            ReasonCode::_TopicFilterInvalid => 143, // DISCONNECT
            ReasonCode::_TopicNameInvalid => 144, // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_PacketIdentifierInUse => 145, // PUBACK
            ReasonCode::_ReceiveMaximumExceeded => 147, // DISCONNECT
            ReasonCode::_TopicAliasInvalid => 148, // DISCONNECT
            ReasonCode::_PacketTooLarge => 149,   // DISCONNECT
            ReasonCode::_MessageRateTooHigh => 150, // DISCONNECT
            ReasonCode::_QuotaExceeded => 151,    // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_AdministrativeAction => 152, // DISCONNECT
            ReasonCode::_PayloadFormatInvalid => 153, // CONNACK - PUBACK - DISCONNECT
            ReasonCode::_RetainNotSupported => 154, // DISCONNECT
            ReasonCode::_QoSNotSupported => 155,  // DISCONNECT
            ReasonCode::_UseAnotherServer => 156, // DISCONNECT
            ReasonCode::_ServerMoved => 157,      // DISCONNECT
            ReasonCode::_SharedSubscriptionsNotSupported => 158, // DISCONNECT
            ReasonCode::_ConnectionRateExceeded => 159, // DISCONNECT
            ReasonCode::_MaximumConnectTime => 160, // DISCONNECT
            ReasonCode::_SubscriptionIdentifiersNotSupported => 161, // DISCONNECT
            ReasonCode::_WildcardSubscriptionsNotSupported => 162, // DISCONNECT
        }
    }

    pub fn new (id: u8) -> Self {
        match id {
            0 => ReasonCode::Success,
            4 => ReasonCode::_DisconnectWithWillMessage,
            16 => ReasonCode::_NoMatchingSubscribers,
            128 => ReasonCode::_UnspecifiedError,
            129 => ReasonCode::_MalformedPacket,
            130 => ReasonCode::_ProtocolError,
            131 => ReasonCode::_ImplementationSpecificError,
            132 => ReasonCode::_UnsupportedProtocolVersion,
            133 => ReasonCode::_ClientIdentifierNotValid,
            134 => ReasonCode::_BadUserNameOrPassword,
            135 => ReasonCode::_NotAuthorized,
            136 => ReasonCode::_ServerUnavailable,
            137 => ReasonCode::_ServerBusy,
            138 => ReasonCode::_Banned,
            139 => ReasonCode::_ServerShuttingDown,
            140 => ReasonCode::_BadAuthenticationMethod,
            141 => ReasonCode::_KeepAliveTimeout,
            142 => ReasonCode::_SessionTakenOver,
            143 => ReasonCode::_TopicFilterInvalid,
            144 => ReasonCode::_TopicNameInvalid,
            145 => ReasonCode::_PacketIdentifierInUse,
            147 => ReasonCode::_ReceiveMaximumExceeded,
            148 => ReasonCode::_TopicAliasInvalid,
            149 => ReasonCode::_PacketTooLarge,
            150 => ReasonCode::_MessageRateTooHigh,
            151 => ReasonCode::_QuotaExceeded,
            152 => ReasonCode::_AdministrativeAction,
            153 => ReasonCode::_PayloadFormatInvalid,
            154 => ReasonCode::_RetainNotSupported,
            155 => ReasonCode::_QoSNotSupported,
            156 => ReasonCode::_UseAnotherServer,
            157 => ReasonCode::_ServerMoved,
            158 => ReasonCode::_SharedSubscriptionsNotSupported,
            159 => ReasonCode::_ConnectionRateExceeded,
            160 => ReasonCode::_MaximumConnectTime,
            161 => ReasonCode::_SubscriptionIdentifiersNotSupported,
            162 => ReasonCode::_WildcardSubscriptionsNotSupported,
            _ => ReasonCode::_UnspecifiedError,
        }
    }
}

impl Display for ReasonCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReasonCode::Success => write!(f, "{} - Success", self.get_id()),
            ReasonCode::_NormalDisconnection => write!(f, "{} - Normal Disconnection", self.get_id()),
            ReasonCode::_DisconnectWithWillMessage => write!(f, "{} - Disconnect With Will Message", self.get_id()),
            ReasonCode::_NoMatchingSubscribers => write!(f, "{} - No Matching Subscribers", self.get_id()),
            ReasonCode::_UnspecifiedError => write!(f, "{} - Unspecified Error", self.get_id()),
            ReasonCode::_MalformedPacket => write!(f, "{} - Malformed Packet", self.get_id()),
            ReasonCode::_ProtocolError => write!(f, "{} - Protocol Error", self.get_id()),
            ReasonCode::_ImplementationSpecificError => write!(f, "{} - Implementation Specific Error", self.get_id()),
            ReasonCode::_UnsupportedProtocolVersion => write!(f, "{} - Unsupported Protocol Version", self.get_id()),
            ReasonCode::_ClientIdentifierNotValid => write!(f, "{} - Client Identifier not valid", self.get_id()),
            ReasonCode::_BadUserNameOrPassword => write!(f, "{} - Bad User Name or Password", self.get_id()),
            ReasonCode::_NotAuthorized => write!(f, "{} - Not authorized", self.get_id()),
            ReasonCode::_ServerUnavailable => write!(f, "{} - Server unavailable", self.get_id()),
            ReasonCode::_ServerBusy => write!(f, "{} - Server busy", self.get_id()),
            ReasonCode::_Banned => write!(f, "{} - Banned", self.get_id()),
            ReasonCode::_ServerShuttingDown => write!(f, "{} - Server shutting down", self.get_id()),
            ReasonCode::_BadAuthenticationMethod => write!(f, "{} - Bad authentication method", self.get_id()),
            ReasonCode::_KeepAliveTimeout => write!(f, "{} - Keep alive timeout", self.get_id()),
            ReasonCode::_SessionTakenOver => write!(f, "{} - Session taken over", self.get_id()),
            ReasonCode::_TopicFilterInvalid => write!(f, "{} - Topic filter invalid", self.get_id()),
            ReasonCode::_TopicNameInvalid => write!(f, "{} - Topic name invalid", self.get_id()),
            ReasonCode::_PacketIdentifierInUse => write!(f, "{} - Packet identifier in use", self.get_id()),
            ReasonCode::_ReceiveMaximumExceeded => write!(f, "{} - Receive maximum exceeded", self.get_id()),
            ReasonCode::_TopicAliasInvalid => write!(f, "{} - Topic alias invalid", self.get_id()),
            ReasonCode::_PacketTooLarge => write!(f, "{} - Packet too large", self.get_id()),
            ReasonCode::_MessageRateTooHigh => write!(f, "{} - Message rate too high", self.get_id()),
            ReasonCode::_QuotaExceeded => write!(f, "{} - Quota exceeded", self.get_id()),
            ReasonCode::_AdministrativeAction => write!(f, "{} - Administrative action", self.get_id()),
            ReasonCode::_PayloadFormatInvalid => write!(f, "{} - Payload format invalid", self.get_id()),
            ReasonCode::_RetainNotSupported => write!(f, "{} - Retain not supported", self.get_id()),
            ReasonCode::_QoSNotSupported => write!(f, "{} - QoS not supported", self.get_id()),
            ReasonCode::_UseAnotherServer => write!(f, "{} - Use another server", self.get_id()),
            ReasonCode::_ServerMoved => write!(f, "{} - Server moved", self.get_id()),
            ReasonCode::_SharedSubscriptionsNotSupported => write!(f, "{} - Shared subscriptions not supported", self.get_id()),
            ReasonCode::_ConnectionRateExceeded => write!(f, "{} - Connection rate exceeded", self.get_id()),
            ReasonCode::_MaximumConnectTime => write!(f, "{} - Maximum connect time", self.get_id()),
            ReasonCode::_SubscriptionIdentifiersNotSupported => write!(f, "{} - Subscription identifiers not supported", self.get_id()),
            ReasonCode::_WildcardSubscriptionsNotSupported => write!(f, "{} - Wildcard subscriptions not supported", self.get_id()),
        }
    }
}
