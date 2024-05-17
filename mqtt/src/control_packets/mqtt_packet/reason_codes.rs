/// ## Reason Codes for MQTT Packets
///
/// #### 0 - 0x00 - Success
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
#[allow(dead_code)]
pub enum ReasonMode {
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

impl ReasonMode {
    pub fn get_id(&self) -> u8 {
        match *self {
            ReasonMode::Success => 0,             // CONNACK, PUBACK, UNSUBACK, AUTH
            ReasonMode::NormalDisconnection => 0, // DISCONNECT
            ReasonMode::GrantedQoS0 => 0,         // SUBACK
            ReasonMode::GrantedQoS1 => 1,         // SUBACK
            ReasonMode::GrantedQoS2 => 2,         // SUBACK
            ReasonMode::DisconnectWithWillMessage => 4, // DISCONNECT
            ReasonMode::NoMatchingSubscribers => 16, // PUBACK
            ReasonMode::NoSubscriptionExisted => 17, // UNSUBACK
            ReasonMode::ContinueAuthentication => 24, // AUTH
            ReasonMode::ReAuthenticate => 25,     // AUTH
            ReasonMode::UnspecifiedError => 128,  // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::MalformedPacket => 129,   // CONNACK, DISCONNECT
            ReasonMode::ProtocolError => 130,     // CONNACK, DISCONNECT
            ReasonMode::ImplementationSpecificError => 131, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::UnsupportedProtocolVersion => 132,  // CONNACK
            ReasonMode::ClientIdentifierNotValid => 133,    // CONNACK
            ReasonMode::BadUserNameOrPassword => 134,       // CONNACK
            ReasonMode::NotAuthorized => 135, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::ServerUnavailable => 136, // CONNACK
            ReasonMode::ServerBusy => 137,    // CONNACK, DISCONNECT
            ReasonMode::Banned => 138,        // CONNACK
            ReasonMode::ServerShuttingDown => 139, // DISCONNECT
            ReasonMode::BadAuthenticationMethod => 140, // CONNACK, DISCONNECT
            ReasonMode::KeepAliveTimeout => 141, // DISCONNECT
            ReasonMode::SessionTakenOver => 142, // DISCONNECT
            ReasonMode::TopicFilterInvalid => 143, // SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::TopicNameInvalid => 144, // CONNACK, PUBACK, DISCONNECT
            ReasonMode::PacketIdentifierInUse => 145, // PUBACK, SUBACK, UNSUBACK
            ReasonMode::ReceiveMaximumExceeded => 147, // DISCONNECT
            ReasonMode::TopicAliasInvalid => 148, // DISCONNECT
            ReasonMode::PacketTooLarge => 149, // CONNACK, DISCONNECT
            ReasonMode::MessageRateTooHigh => 150, // DISCONNECT
            ReasonMode::QuotaExceeded => 151, // CONNACK, PUBACK, SUBACK, DISCONNECT
            ReasonMode::AdministrativeAction => 152, // DISCONNECT
            ReasonMode::PayloadFormatInvalid => 153, // CONNACK, PUBACK, DISCONNECT
            ReasonMode::RetainNotSupported => 154, // CONNACK, DISCONNECT
            ReasonMode::QoSNotSupported => 155, // CONNACK, DISCONNECT
            ReasonMode::UseAnotherServer => 156, // CONNACK, DISCONNECT
            ReasonMode::ServerMoved => 157,   // CONNACK, DISCONNECT
            ReasonMode::SharedSubscriptionsNotSupported => 158, // SUBACK, DISCONNECT
            ReasonMode::ConnectionRateExceeded => 159, // CONNACK, DISCONNECT
            ReasonMode::MaximumConnectTime => 160, // DISCONNECT
            ReasonMode::SubscriptionIdentifiersNotSupported => 161, // SUBACK, DISCONNECT
            ReasonMode::WildcardSubscriptionsNotSupported => 162, // SUBACK, DISCONNECT
        }
    }
}
