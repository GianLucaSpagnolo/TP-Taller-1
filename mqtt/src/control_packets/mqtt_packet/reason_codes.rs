use std::fmt::Display;

/// Connect Reason Code
/// Byte 2 in the Variable Header is the Connect Reason Code.
/// 0 - 0x00 - Success
/// The Connection is accepted.
///
/// #### 0 - 0x00 - Normal disconnection
/// Close the connection normally. Do not send the Will Message.
/// If there was a Will Message, the Client wishes to disconnect but requires that the Server also publishes its Will Message.
///
/// ### 0 - 0x00 - Granted QoS 0
/// The subscription is accepted and the maximum QoS sent will be QoS 0.
///
/// #### 1 - 0x01 - Granted QoS 1
/// The subscription is accepted and the maximum QoS sent will be QoS 1.
///
/// #### 2 - 0x02 - Granted QoS 2
/// The subscription is accepted and the maximum QoS sent will be QoS 2.
///
/// #### 4 - 0x04 - Disconnect with Will Message
/// The Client wishes to disconnect and requires that the Server also publishes its Will Message.
///
/// #### 16 - 0x10 - No matching subscribers
/// No matching subscribers. The Client or Server will not forward the PUBLISH packet.
///
/// #### 17 - 0x11 - No subscription existed
/// There is no matching topic filter, and the Server does not accept any new subscriptions.
///
/// #### 24 - 0x18 - Continue authentication
/// Continue the authentication with another step.
///
/// #### 25 - 0x19 - Re-authenticate
/// Initiate a re-authentication.
///
/// #### 128 - 0x80 - Unspecified error
/// The Connection is closed but the sender either does not wish to reveal the reason, or none of the other Reason Codes apply.
///
/// #### 129 - 0x81 - Malformed Packet
/// The received packet does not conform to this specification.
///
/// #### 130 - 0x82 - Protocol Error
/// An unexpected or out of order packet was received.
///
/// #### 131 - 0x83 - Implementation specific error
/// The packet received is valid but cannot be processed by this implementation.
///
/// #### 132 - 0x84 - Unsupported Protocol Version
/// The Server does not support the version of the MQTT protocol requested by the Client.
///
/// #### 133 - 0x85 - Client Identifier not valid
/// The Client Identifier is a valid string but is not allowed by the Server.
///
/// #### 134 - 0x86 - Bad User Name or Password
/// The Server does not accept the User Name or Password specified by the Client.
///
/// #### 135 - 0x87 - Not authorized
/// There is no authorization to do the operation determinated by the packet.
///
/// #### 136 - 0x88 - Server unavailable
/// The MQTT Server is not available.
///
/// #### 137 - 0x89 - Server busy
/// The Server is busy. Try again later.
///
/// #### 138 - 0x8A - Banned
/// This Client has been banned by administrative action. Contact the server administrator.
///
/// #### 139 - 0x8B - Server shutting down
/// The Server is shutting down.
///
/// #### 140 - 0x8C - Bad authentication method
/// The authentication method is not supported or does not match the authentication method currently in use.
///
/// #### 141 - 0x8D - Keep Alive timeout
/// The Connection is closed because no packet has been received for 1.5 times the Keepalive time.
///
/// #### 142 - 0x8E - Session taken over
/// Another Connection using the same ClientID has connected causing this Connection to be closed.
///
/// #### 143 - 0x8F - Topic Filter invalid
/// The Topic Filter is correctly formed, but is not accepted by this Sever.
///
/// #### 144 - 0x90 - Topic Name invalid
/// The Topic Name is correctly formed, but is not accepted by this Client or Server.
///
/// #### 145 - 0x91 - Packet Identifier in use
/// The Packet Identifier is already in use. This might indicate a mismatch in the Session State between the Client and Server.
///
/// #### 147 - 0x93 - Receive Maximum exceeded
/// The Client or Server has received more than Receive Maximum publication for which it has not sent PUBACK.
///
/// #### 148 - 0x94 - Topic Alias invalid
/// The Client or Server has received a PUBLISH packet containing a Topic Alias which is greater than the Maximum Topic Alias it sent in the CONNECT or CONNACK packet.
///
/// #### 149 - 0x95 - Packet too large
/// The packet size is greater than Maximum Packet Size for this Client or Server.
///
/// #### 150 - 0x96 - Message rate too high
/// The received data rate is too high.
///
/// #### 151 - 0x97 - Quota exceeded
/// An implementation or administrative imposed limit has been exceeded.
///
/// #### 152 - 0x98 - Administrative action
/// The Connection is closed due to an administrative action.
///
/// #### 153 - 0x99 - Payload format invalid
/// The payload format does not match the one specified by the Payload Format Indicator.
///
/// #### 154 - 0x9A - Retain not supported
/// The Server has does not support retained messages.
///
/// #### 155 - 0x9B - QoS not supported
/// The Client specified a QoS greater than the QoS specified in a Maximum QoS in the CONNACK.
///
/// #### 156 - 0x9C - Use another server
/// The Client should temporarily change its Server.
///
/// #### 157 - 0x9D - Server moved
/// The Server is moved and the Client should permanently change its server location.
///
/// #### 158 - 0x9E - Shared subscriptions not supported
/// The Server does not support Shared Subscriptions.
///
/// #### 159 - 0x9F - Connection rate exceeded
/// This connection is closed because the connection rate is too high.
///
/// #### 160 - 0xA0 - Maximum connect time
/// The maximum connection time authorized for this connection has been exceeded.
///
/// #### 161 - 0xA1 - Subscription identifiers not supported
/// The Server does not support Subscription Identifiers; the subscription is not accepted.
///
/// #### 162 - 0xA2 - Wildcard subscriptions not supported
/// The Server does not support Wildcard Subscriptions; the subscription is not accepted.
///
pub enum ReasonCode {
    Success,
    NormalDisconnection,
    GrantedQoS0,
    GrantedQoS1,
    GrantedQoS2,
    DisconnectWithWillMessage,
    NoMatchingSubscribers,
    NoSubscriptionExisted,
    ContinueAuthentication,
    ReAuthenticate,
    UnspecifiedError,
    MalformedPacket,
    ProtocolError,
    ImplementationSpecificError,
    UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    BadUserNameOrPassword,
    NotAuthorized,
    ServerUnavailable,
    ServerBusy,
    Banned,
    ServerShuttingDown,
    BadAuthenticationMethod,
    KeepAliveTimeout,
    SessionTakenOver,
    TopicFilterInvalid,
    TopicNameInvalid,
    PacketIdentifierInUse,
    ReceiveMaximumExceeded,
    TopicAliasInvalid,
    PacketTooLarge,
    MessageRateTooHigh,
    QuotaExceeded,
    AdministrativeAction,
    PayloadFormatInvalid,
    RetainNotSupported,
    QoSNotSupported,
    UseAnotherServer,
    ServerMoved,
    SharedSubscriptionsNotSupported,
    ConnectionRateExceeded,
    MaximumConnectTime,
    SubscriptionIdentifiersNotSupported,
    WildcardSubscriptionsNotSupported,
}

impl ReasonCode {

    /// ## get_id
    /// 
    /// Devuelve el id del Reason Code
    /// 
    /// ### Retorno
    /// - `u8`: id del Reason Code
    /// 
    pub fn get_id(&self) -> u8 {
        match *self {
            ReasonCode::Success => 0,             // CONNACK, PUBACK, UNSUBACK, AUTH
            ReasonCode::NormalDisconnection => 0, // DISCONNECT
            ReasonCode::GrantedQoS0 => 0,         // SUBACK
            ReasonCode::GrantedQoS1 => 1,         // SUBACK
            ReasonCode::GrantedQoS2 => 2,         // SUBACK
            ReasonCode::DisconnectWithWillMessage => 4, // DISCONNECT
            ReasonCode::NoMatchingSubscribers => 16, // PUBACK
            ReasonCode::NoSubscriptionExisted => 17, // UNSUBACK
            ReasonCode::ContinueAuthentication => 24, // AUTH
            ReasonCode::ReAuthenticate => 25,     // AUTH
            ReasonCode::UnspecifiedError => 128,  // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonCode::MalformedPacket => 129,   // CONNACK, DISCONNECT
            ReasonCode::ProtocolError => 130,     // CONNACK, DISCONNECT
            ReasonCode::ImplementationSpecificError => 131, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonCode::UnsupportedProtocolVersion => 132,  // CONNACK
            ReasonCode::ClientIdentifierNotValid => 133,    // CONNACK
            ReasonCode::BadUserNameOrPassword => 134,       // CONNACK
            ReasonCode::NotAuthorized => 135, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonCode::ServerUnavailable => 136, // CONNACK
            ReasonCode::ServerBusy => 137,    // CONNACK, DISCONNECT
            ReasonCode::Banned => 138,        // CONNACK
            ReasonCode::ServerShuttingDown => 139, // DISCONNECT
            ReasonCode::BadAuthenticationMethod => 140, // CONNACK, DISCONNECT
            ReasonCode::KeepAliveTimeout => 141, // DISCONNECT
            ReasonCode::SessionTakenOver => 142, // DISCONNECT
            ReasonCode::TopicFilterInvalid => 143, // SUBACK, UNSUBACK, DISCONNECT
            ReasonCode::TopicNameInvalid => 144, // CONNACK, PUBACK, DISCONNECT
            ReasonCode::PacketIdentifierInUse => 145, // PUBACK, SUBACK, UNSUBACK
            ReasonCode::ReceiveMaximumExceeded => 147, // DISCONNECT
            ReasonCode::TopicAliasInvalid => 148, // DISCONNECT
            ReasonCode::PacketTooLarge => 149, // CONNACK, DISCONNECT
            ReasonCode::MessageRateTooHigh => 150, // DISCONNECT
            ReasonCode::QuotaExceeded => 151, // CONNACK, PUBACK, SUBACK, DISCONNECT
            ReasonCode::AdministrativeAction => 152, // DISCONNECT
            ReasonCode::PayloadFormatInvalid => 153, // CONNACK, PUBACK, DISCONNECT
            ReasonCode::RetainNotSupported => 154, // CONNACK, DISCONNECT
            ReasonCode::QoSNotSupported => 155, // CONNACK, DISCONNECT
            ReasonCode::UseAnotherServer => 156, // CONNACK, DISCONNECT
            ReasonCode::ServerMoved => 157,   // CONNACK, DISCONNECT
            ReasonCode::SharedSubscriptionsNotSupported => 158, // SUBACK, DISCONNECT
            ReasonCode::ConnectionRateExceeded => 159, // CONNACK, DISCONNECT
            ReasonCode::MaximumConnectTime => 160, // DISCONNECT
            ReasonCode::SubscriptionIdentifiersNotSupported => 161, // SUBACK, DISCONNECT
            ReasonCode::WildcardSubscriptionsNotSupported => 162, // SUBACK, DISCONNECT
        }
    }

    /// ## new
    /// 
    /// Devuelve un nuevo Reason Code
    /// 
    /// ### Parametros
    /// - `id`: id del Reason Code
    /// 
    /// ### Retorno
    /// - `ReasonCode`: nuevo Reason Code
    pub fn new(id: u8) -> Self {
        match id {
            0 => ReasonCode::Success,
            1 => ReasonCode::GrantedQoS1,
            2 => ReasonCode::GrantedQoS2,
            4 => ReasonCode::DisconnectWithWillMessage,
            16 => ReasonCode::NoMatchingSubscribers,
            17 => ReasonCode::NoSubscriptionExisted,
            24 => ReasonCode::ContinueAuthentication,
            25 => ReasonCode::ReAuthenticate,
            128 => ReasonCode::UnspecifiedError,
            129 => ReasonCode::MalformedPacket,
            130 => ReasonCode::ProtocolError,
            131 => ReasonCode::ImplementationSpecificError,
            132 => ReasonCode::UnsupportedProtocolVersion,
            133 => ReasonCode::ClientIdentifierNotValid,
            134 => ReasonCode::BadUserNameOrPassword,
            135 => ReasonCode::NotAuthorized,
            136 => ReasonCode::ServerUnavailable,
            137 => ReasonCode::ServerBusy,
            138 => ReasonCode::Banned,
            139 => ReasonCode::ServerShuttingDown,
            140 => ReasonCode::BadAuthenticationMethod,
            141 => ReasonCode::KeepAliveTimeout,
            142 => ReasonCode::SessionTakenOver,
            143 => ReasonCode::TopicFilterInvalid,
            144 => ReasonCode::TopicNameInvalid,
            145 => ReasonCode::PacketIdentifierInUse,
            147 => ReasonCode::ReceiveMaximumExceeded,
            148 => ReasonCode::TopicAliasInvalid,
            149 => ReasonCode::PacketTooLarge,
            150 => ReasonCode::MessageRateTooHigh,
            151 => ReasonCode::QuotaExceeded,
            152 => ReasonCode::AdministrativeAction,
            153 => ReasonCode::PayloadFormatInvalid,
            154 => ReasonCode::RetainNotSupported,
            155 => ReasonCode::QoSNotSupported,
            156 => ReasonCode::UseAnotherServer,
            157 => ReasonCode::ServerMoved,
            158 => ReasonCode::SharedSubscriptionsNotSupported,
            159 => ReasonCode::ConnectionRateExceeded,
            160 => ReasonCode::MaximumConnectTime,
            161 => ReasonCode::SubscriptionIdentifiersNotSupported,
            162 => ReasonCode::WildcardSubscriptionsNotSupported,
            _ => ReasonCode::UnspecifiedError,
        }
    }


    /// ## is_valid_disconnect_code_from_server
    /// 
    /// Verifica si el Reason Code es valido
    /// para ser enviado por el servidor
    /// 
    /// ### Retorno
    /// - `bool`: true si es valido, false si no
    ///     
    pub fn is_valid_disconnect_code_from_server(&self) -> bool {
        matches!(
            *self,
            ReasonCode::NormalDisconnection
                | ReasonCode::UnspecifiedError
                | ReasonCode::MalformedPacket
                | ReasonCode::ProtocolError
                | ReasonCode::ImplementationSpecificError
                | ReasonCode::NotAuthorized
                | ReasonCode::ServerBusy
                | ReasonCode::ServerShuttingDown
                | ReasonCode::KeepAliveTimeout
                | ReasonCode::SessionTakenOver
                | ReasonCode::TopicFilterInvalid
                | ReasonCode::TopicNameInvalid
                | ReasonCode::ReceiveMaximumExceeded
                | ReasonCode::TopicAliasInvalid
                | ReasonCode::PacketTooLarge
                | ReasonCode::MessageRateTooHigh
                | ReasonCode::QuotaExceeded
                | ReasonCode::AdministrativeAction
                | ReasonCode::PayloadFormatInvalid
                | ReasonCode::RetainNotSupported
                | ReasonCode::QoSNotSupported
                | ReasonCode::UseAnotherServer
                | ReasonCode::ServerMoved
                | ReasonCode::SharedSubscriptionsNotSupported
                | ReasonCode::ConnectionRateExceeded
                | ReasonCode::MaximumConnectTime
                | ReasonCode::SubscriptionIdentifiersNotSupported
                | ReasonCode::WildcardSubscriptionsNotSupported
        )
    }

    /// ## is_valid_disconnect_code_from_client
    /// 
    /// Verifica si el Reason Code es valido
    /// para ser enviado por el cliente
    /// 
    /// ### Retorno
    /// - `bool`: true si es valido, false si no
    /// 
    pub fn is_valid_disconnect_code_from_client(&self) -> bool {
        matches!(
            *self,
            ReasonCode::NormalDisconnection
                | ReasonCode::DisconnectWithWillMessage
                | ReasonCode::UnspecifiedError
                | ReasonCode::MalformedPacket
                | ReasonCode::ProtocolError
                | ReasonCode::ImplementationSpecificError
                | ReasonCode::TopicNameInvalid
                | ReasonCode::ReceiveMaximumExceeded
                | ReasonCode::TopicAliasInvalid
                | ReasonCode::PacketTooLarge
                | ReasonCode::MessageRateTooHigh
                | ReasonCode::QuotaExceeded
                | ReasonCode::AdministrativeAction
                | ReasonCode::PayloadFormatInvalid
        )
    }
}

impl Display for ReasonCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ReasonCode::Success => write!(f, "{} - Success", self.get_id()),
            ReasonCode::NormalDisconnection => {
                write!(f, "{} - Normal Disconnection", self.get_id())
            }
            ReasonCode::GrantedQoS0 => write!(f, "{} - Granted QoS 0", self.get_id()),
            ReasonCode::GrantedQoS1 => write!(f, "{} - Granted QoS 1", self.get_id()),
            ReasonCode::GrantedQoS2 => write!(f, "{} - Granted QoS 2", self.get_id()),
            ReasonCode::DisconnectWithWillMessage => {
                write!(f, "{} - Disconnect With Will Message", self.get_id())
            }
            ReasonCode::NoMatchingSubscribers => {
                write!(f, "{} - No Matching Subscribers", self.get_id())
            }
            ReasonCode::NoSubscriptionExisted => {
                write!(f, "{} - No Subscription Existed", self.get_id())
            }
            ReasonCode::ContinueAuthentication => {
                write!(f, "{} - Continue Authentication", self.get_id())
            }
            ReasonCode::ReAuthenticate => write!(f, "{} - Re-authenticate", self.get_id()),
            ReasonCode::UnspecifiedError => write!(f, "{} - Unspecified Error", self.get_id()),
            ReasonCode::MalformedPacket => write!(f, "{} - Malformed Packet", self.get_id()),
            ReasonCode::ProtocolError => write!(f, "{} - Protocol Error", self.get_id()),
            ReasonCode::ImplementationSpecificError => {
                write!(f, "{} - Implementation Specific Error", self.get_id())
            }
            ReasonCode::UnsupportedProtocolVersion => {
                write!(f, "{} - Unsupported Protocol Version", self.get_id())
            }
            ReasonCode::ClientIdentifierNotValid => {
                write!(f, "{} - Client Identifier not valid", self.get_id())
            }
            ReasonCode::BadUserNameOrPassword => {
                write!(f, "{} - Bad User Name or Password", self.get_id())
            }
            ReasonCode::NotAuthorized => write!(f, "{} - Not authorized", self.get_id()),
            ReasonCode::ServerUnavailable => write!(f, "{} - Server unavailable", self.get_id()),
            ReasonCode::ServerBusy => write!(f, "{} - Server busy", self.get_id()),
            ReasonCode::Banned => write!(f, "{} - Banned", self.get_id()),
            ReasonCode::ServerShuttingDown => {
                write!(f, "{} - Server shutting down", self.get_id())
            }
            ReasonCode::BadAuthenticationMethod => {
                write!(f, "{} - Bad authentication method", self.get_id())
            }
            ReasonCode::KeepAliveTimeout => write!(f, "{} - Keep alive timeout", self.get_id()),
            ReasonCode::SessionTakenOver => write!(f, "{} - Session taken over", self.get_id()),
            ReasonCode::TopicFilterInvalid => {
                write!(f, "{} - Topic filter invalid", self.get_id())
            }
            ReasonCode::TopicNameInvalid => write!(f, "{} - Topic name invalid", self.get_id()),
            ReasonCode::PacketIdentifierInUse => {
                write!(f, "{} - Packet identifier in use", self.get_id())
            }
            ReasonCode::ReceiveMaximumExceeded => {
                write!(f, "{} - Receive maximum exceeded", self.get_id())
            }
            ReasonCode::TopicAliasInvalid => write!(f, "{} - Topic alias invalid", self.get_id()),
            ReasonCode::PacketTooLarge => write!(f, "{} - Packet too large", self.get_id()),
            ReasonCode::MessageRateTooHigh => {
                write!(f, "{} - Message rate too high", self.get_id())
            }
            ReasonCode::QuotaExceeded => write!(f, "{} - Quota exceeded", self.get_id()),
            ReasonCode::AdministrativeAction => {
                write!(f, "{} - Administrative action", self.get_id())
            }
            ReasonCode::PayloadFormatInvalid => {
                write!(f, "{} - Payload format invalid", self.get_id())
            }
            ReasonCode::RetainNotSupported => {
                write!(f, "{} - Retain not supported", self.get_id())
            }
            ReasonCode::QoSNotSupported => write!(f, "{} - QoS not supported", self.get_id()),
            ReasonCode::UseAnotherServer => write!(f, "{} - Use another server", self.get_id()),
            ReasonCode::ServerMoved => write!(f, "{} - Server moved", self.get_id()),
            ReasonCode::SharedSubscriptionsNotSupported => {
                write!(f, "{} - Shared subscriptions not supported", self.get_id())
            }
            ReasonCode::ConnectionRateExceeded => {
                write!(f, "{} - Connection rate exceeded", self.get_id())
            }
            ReasonCode::MaximumConnectTime => {
                write!(f, "{} - Maximum connect time", self.get_id())
            }
            ReasonCode::SubscriptionIdentifiersNotSupported => write!(
                f,
                "{} - Subscription identifiers not supported",
                self.get_id()
            ),
            ReasonCode::WildcardSubscriptionsNotSupported => write!(
                f,
                "{} - Wildcard subscriptions not supported",
                self.get_id()
            ),
        }
    }
}
