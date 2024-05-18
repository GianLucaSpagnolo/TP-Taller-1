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

pub enum ReasonMode {
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

impl ReasonMode {
    pub fn get_id(&self) -> u8 {
        match *self {
            ReasonMode::Success => 0,
            ReasonMode::_NormalDisconnection => 0, // DISCONNECT
            ReasonMode::_DisconnectWithWillMessage => 4, // DISCONNECT
            ReasonMode::_NoMatchingSubscribers => 16, // PUBACK
            ReasonMode::_UnspecifiedError => 128,  // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_MalformedPacket => 129,   // DISCONNECT
            ReasonMode::_ProtocolError => 130,     // DISCONNECT
            ReasonMode::_ImplementationSpecificError => 131, // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_UnsupportedProtocolVersion => 132,
            ReasonMode::_ClientIdentifierNotValid => 133,
            ReasonMode::_BadUserNameOrPassword => 134,
            ReasonMode::_NotAuthorized => 135, // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_ServerUnavailable => 136,
            ReasonMode::_ServerBusy => 137, // DISCONNECT
            ReasonMode::_Banned => 138,
            ReasonMode::_ServerShuttingDown => 139, // DISCONNECT
            ReasonMode::_BadAuthenticationMethod => 140,
            ReasonMode::_KeepAliveTimeout => 141, // DISCONNECT
            ReasonMode::_SessionTakenOver => 142, // DISCONNECT
            ReasonMode::_TopicFilterInvalid => 143, // DISCONNECT
            ReasonMode::_TopicNameInvalid => 144, // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_PacketIdentifierInUse => 145, // PUBACK
            ReasonMode::_ReceiveMaximumExceeded => 147, // DISCONNECT
            ReasonMode::_TopicAliasInvalid => 148, // DISCONNECT
            ReasonMode::_PacketTooLarge => 149,   // DISCONNECT
            ReasonMode::_MessageRateTooHigh => 150, // DISCONNECT
            ReasonMode::_QuotaExceeded => 151,    // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_AdministrativeAction => 152, // DISCONNECT
            ReasonMode::_PayloadFormatInvalid => 153, // CONNACK - PUBACK - DISCONNECT
            ReasonMode::_RetainNotSupported => 154, // DISCONNECT
            ReasonMode::_QoSNotSupported => 155,  // DISCONNECT
            ReasonMode::_UseAnotherServer => 156, // DISCONNECT
            ReasonMode::_ServerMoved => 157,      // DISCONNECT
            ReasonMode::_SharedSubscriptionsNotSupported => 158, // DISCONNECT
            ReasonMode::_ConnectionRateExceeded => 159, // DISCONNECT
            ReasonMode::_MaximumConnectTime => 160, // DISCONNECT
            ReasonMode::_SubscriptionIdentifiersNotSupported => 161, // DISCONNECT
            ReasonMode::_WildcardSubscriptionsNotSupported => 162, // DISCONNECT
        }
    }

    pub fn new (id: u8) -> Self {
        match id {
            0 => ReasonMode::Success,
            4 => ReasonMode::_DisconnectWithWillMessage,
            16 => ReasonMode::_NoMatchingSubscribers,
            128 => ReasonMode::_UnspecifiedError,
            129 => ReasonMode::_MalformedPacket,
            130 => ReasonMode::_ProtocolError,
            131 => ReasonMode::_ImplementationSpecificError,
            132 => ReasonMode::_UnsupportedProtocolVersion,
            133 => ReasonMode::_ClientIdentifierNotValid,
            134 => ReasonMode::_BadUserNameOrPassword,
            135 => ReasonMode::_NotAuthorized,
            136 => ReasonMode::_ServerUnavailable,
            137 => ReasonMode::_ServerBusy,
            138 => ReasonMode::_Banned,
            139 => ReasonMode::_ServerShuttingDown,
            140 => ReasonMode::_BadAuthenticationMethod,
            141 => ReasonMode::_KeepAliveTimeout,
            142 => ReasonMode::_SessionTakenOver,
            143 => ReasonMode::_TopicFilterInvalid,
            144 => ReasonMode::_TopicNameInvalid,
            145 => ReasonMode::_PacketIdentifierInUse,
            147 => ReasonMode::_ReceiveMaximumExceeded,
            148 => ReasonMode::_TopicAliasInvalid,
            149 => ReasonMode::_PacketTooLarge,
            150 => ReasonMode::_MessageRateTooHigh,
            151 => ReasonMode::_QuotaExceeded,
            152 => ReasonMode::_AdministrativeAction,
            153 => ReasonMode::_PayloadFormatInvalid,
            154 => ReasonMode::_RetainNotSupported,
            155 => ReasonMode::_QoSNotSupported,
            156 => ReasonMode::_UseAnotherServer,
            157 => ReasonMode::_ServerMoved,
            158 => ReasonMode::_SharedSubscriptionsNotSupported,
            159 => ReasonMode::_ConnectionRateExceeded,
            160 => ReasonMode::_MaximumConnectTime,
            161 => ReasonMode::_SubscriptionIdentifiersNotSupported,
            162 => ReasonMode::_WildcardSubscriptionsNotSupported,
            _ => ReasonMode::_UnspecifiedError,
        }
    }
}

impl Display for ReasonMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReasonMode::Success => write!(f, "{} - Success", self.get_id()),
            ReasonMode::_NormalDisconnection => write!(f, "{} - Normal Disconnection", self.get_id()),
            ReasonMode::_DisconnectWithWillMessage => write!(f, "{} - Disconnect With Will Message", self.get_id()),
            ReasonMode::_NoMatchingSubscribers => write!(f, "{} - No Matching Subscribers", self.get_id()),
            ReasonMode::_UnspecifiedError => write!(f, "{} - Unspecified Error", self.get_id()),
            ReasonMode::_MalformedPacket => write!(f, "{} - Malformed Packet", self.get_id()),
            ReasonMode::_ProtocolError => write!(f, "{} - Protocol Error", self.get_id()),
            ReasonMode::_ImplementationSpecificError => write!(f, "{} - Implementation Specific Error", self.get_id()),
            ReasonMode::_UnsupportedProtocolVersion => write!(f, "{} - Unsupported Protocol Version", self.get_id()),
            ReasonMode::_ClientIdentifierNotValid => write!(f, "{} - Client Identifier not valid", self.get_id()),
            ReasonMode::_BadUserNameOrPassword => write!(f, "{} - Bad User Name or Password", self.get_id()),
            ReasonMode::_NotAuthorized => write!(f, "{} - Not authorized", self.get_id()),
            ReasonMode::_ServerUnavailable => write!(f, "{} - Server unavailable", self.get_id()),
            ReasonMode::_ServerBusy => write!(f, "{} - Server busy", self.get_id()),
            ReasonMode::_Banned => write!(f, "{} - Banned", self.get_id()),
            ReasonMode::_ServerShuttingDown => write!(f, "{} - Server shutting down", self.get_id()),
            ReasonMode::_BadAuthenticationMethod => write!(f, "{} - Bad authentication method", self.get_id()),
            ReasonMode::_KeepAliveTimeout => write!(f, "{} - Keep alive timeout", self.get_id()),
            ReasonMode::_SessionTakenOver => write!(f, "{} - Session taken over", self.get_id()),
            ReasonMode::_TopicFilterInvalid => write!(f, "{} - Topic filter invalid", self.get_id()),
            ReasonMode::_TopicNameInvalid => write!(f, "{} - Topic name invalid", self.get_id()),
            ReasonMode::_PacketIdentifierInUse => write!(f, "{} - Packet identifier in use", self.get_id()),
            ReasonMode::_ReceiveMaximumExceeded => write!(f, "{} - Receive maximum exceeded", self.get_id()),
            ReasonMode::_TopicAliasInvalid => write!(f, "{} - Topic alias invalid", self.get_id()),
            ReasonMode::_PacketTooLarge => write!(f, "{} - Packet too large", self.get_id()),
            ReasonMode::_MessageRateTooHigh => write!(f, "{} - Message rate too high", self.get_id()),
            ReasonMode::_QuotaExceeded => write!(f, "{} - Quota exceeded", self.get_id()),
            ReasonMode::_AdministrativeAction => write!(f, "{} - Administrative action", self.get_id()),
            ReasonMode::_PayloadFormatInvalid => write!(f, "{} - Payload format invalid", self.get_id()),
            ReasonMode::_RetainNotSupported => write!(f, "{} - Retain not supported", self.get_id()),
            ReasonMode::_QoSNotSupported => write!(f, "{} - QoS not supported", self.get_id()),
            ReasonMode::_UseAnotherServer => write!(f, "{} - Use another server", self.get_id()),
            ReasonMode::_ServerMoved => write!(f, "{} - Server moved", self.get_id()),
            ReasonMode::_SharedSubscriptionsNotSupported => write!(f, "{} - Shared subscriptions not supported", self.get_id()),
            ReasonMode::_ConnectionRateExceeded => write!(f, "{} - Connection rate exceeded", self.get_id()),
            ReasonMode::_MaximumConnectTime => write!(f, "{} - Maximum connect time", self.get_id()),
            ReasonMode::_SubscriptionIdentifiersNotSupported => write!(f, "{} - Subscription identifiers not supported", self.get_id()),
            ReasonMode::_WildcardSubscriptionsNotSupported => write!(f, "{} - Wildcard subscriptions not supported", self.get_id()),
        }
    }
}
