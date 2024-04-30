pub mod client;
pub mod server;
mod control_packets {
    pub mod mqtt_connect {
        pub mod connect;
        pub mod connect_payload;
        pub mod variable_header;
    }
    pub mod mqtt_connack {
        pub mod connack;
        pub mod connect_reason_code;
        pub mod variable_header;
    }

    pub mod mqtt_packet {
        pub mod fixed_header;
        pub mod variable_header_properties;
        pub mod variable_header_property;
    }
}

mod data_structures {
    pub mod data_types;
}
