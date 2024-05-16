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
pub enum ReasonMode {
    Success,
    _NormalDisconnection,
    _GrantedQoS0,
    _GrantedQoS1,
    _GrantedQoS2,
    _DisconnectWithWillMessage,
    _NoMatchingSubscribers,
    _NoSubscriptionExisted,
    ContinueAuthentication,
    ReAuthenticate,
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
            ReasonMode::Success => 0, // CONNACK, PUBACK, UNSUBACK, AUTH
            ReasonMode::_NormalDisconnection => 0, // DISCONNECT
            ReasonMode::_GrantedQoS0 => 0, // SUBACK
            ReasonMode::_GrantedQoS1 => 1, // SUBACK
            ReasonMode::_GrantedQoS2 => 2, // SUBACK
            ReasonMode::_DisconnectWithWillMessage => 4, // DISCONNECT
            ReasonMode::_NoMatchingSubscribers => 16, // PUBACK
            ReasonMode::_NoSubscriptionExisted => 17, // UNSUBACK
            ReasonMode::ContinueAuthentication => 24, // AUTH
            ReasonMode::ReAuthenticate => 25, // AUTH
            ReasonMode::_UnspecifiedError => 128,  // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::_MalformedPacket => 129,   // CONNACK, DISCONNECT
            ReasonMode::_ProtocolError => 130,     // CONNACK, DISCONNECT
            ReasonMode::_ImplementationSpecificError => 131, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::_UnsupportedProtocolVersion => 132, // CONNACK
            ReasonMode::_ClientIdentifierNotValid => 133, // CONNACK
            ReasonMode::_BadUserNameOrPassword => 134, // CONNACK
            ReasonMode::_NotAuthorized => 135, // CONNACK, PUBACK, SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::_ServerUnavailable => 136, // CONNACK
            ReasonMode::_ServerBusy => 137, // CONNACK, DISCONNECT
            ReasonMode::_Banned => 138, // CONNACK
            ReasonMode::_ServerShuttingDown => 139, // DISCONNECT
            ReasonMode::_BadAuthenticationMethod => 140, // CONNACK, DISCONNECT
            ReasonMode::_KeepAliveTimeout => 141, // DISCONNECT
            ReasonMode::_SessionTakenOver => 142, // DISCONNECT
            ReasonMode::_TopicFilterInvalid => 143, // SUBACK, UNSUBACK, DISCONNECT
            ReasonMode::_TopicNameInvalid => 144, // CONNACK, PUBACK, DISCONNECT
            ReasonMode::_PacketIdentifierInUse => 145, // PUBACK, SUBACK, UNSUBACK
            ReasonMode::_ReceiveMaximumExceeded => 147, // DISCONNECT
            ReasonMode::_TopicAliasInvalid => 148, // DISCONNECT
            ReasonMode::_PacketTooLarge => 149,   // CONNACK, DISCONNECT
            ReasonMode::_MessageRateTooHigh => 150, // DISCONNECT
            ReasonMode::_QuotaExceeded => 151,    // CONNACK, PUBACK, SUBACK, DISCONNECT
            ReasonMode::_AdministrativeAction => 152, // DISCONNECT
            ReasonMode::_PayloadFormatInvalid => 153, // CONNACK, PUBACK, DISCONNECT
            ReasonMode::_RetainNotSupported => 154, // CONNACK, DISCONNECT
            ReasonMode::_QoSNotSupported => 155,  // CONNACK, DISCONNECT
            ReasonMode::_UseAnotherServer => 156, // CONNACK, DISCONNECT
            ReasonMode::_ServerMoved => 157,      // CONNACK, DISCONNECT
            ReasonMode::_SharedSubscriptionsNotSupported => 158, // SUBACK, DISCONNECT
            ReasonMode::_ConnectionRateExceeded => 159, // CONNACK, DISCONNECT
            ReasonMode::_MaximumConnectTime => 160, // DISCONNECT
            ReasonMode::_SubscriptionIdentifiersNotSupported => 161, // SUBACK, DISCONNECT
            ReasonMode::_WildcardSubscriptionsNotSupported => 162, // SUBACK, DISCONNECT
        }
    }
}
