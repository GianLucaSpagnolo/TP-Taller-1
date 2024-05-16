use std::{
    io::{Error, Read},
    net::TcpStream,
    sync::{RwLock, RwLockReadGuard},
    thread,
};

use crate::{
    actions::MqttActions,
    config::{ClientConfig, Config},
    control_packets::{
        mqtt_connack::connack::Connack,
        mqtt_connect::{connect::Connect, payload},
        mqtt_packet::fixed_header::PacketFixedHeader,
        mqtt_packet::packet::generic_packet::*,
    },
};
pub struct MqttClient {
    id: String,
    config: ClientConfig,
    stream: TcpStream,
}

fn handle_connack_packet(mut stream: &mut TcpStream) -> Result<Connack, Error> {
    let fixed_header = PacketFixedHeader::read_from(&mut stream)?;

    let packet_recived = get_packet(
        stream,
        fixed_header.get_package_type(),
        fixed_header.remaining_length,
    )?;

    match packet_recived {
        PacketReceived::Connack(ack) => Ok(*ack),
        _ => Err(Error::new(
            std::io::ErrorKind::Other,
            "ClientReceive - Paquete desconocido",
        )),
    }
}

impl MqttClient {
    pub fn init(id: String, config: ClientConfig) -> Result<Self, Error> {
        let mut stream = TcpStream::connect(config.get_socket_address())?;

        let payload = payload::ConnectPayload {
            client_id: id.clone(),
            ..Default::default()
        };

        let connection = Connect::new(config.connect_properties.clone(), payload);
        match connection.send(&mut stream) {
            Ok(_) => println!("Client connection successful"),
            Err(e) => {
                eprintln!("Client connection failure");
                return Err(e);
            }
        };

        let connack = match handle_connack_packet(&mut stream) {
            Ok(r) => {
                println!("Connack handler finish");
                r
            }
            Err(e) => {
                eprintln!("Connack packet handler fails");
                return Err(e);
            }
        };

        /*
        // el servidor loggea, no el cliente
        MqttActions::ClientConnection(
            config.get_socket_address().to_string(),
            connack.properties.connect_reason_code,
        )
        .register_action();
        */
        Ok(MqttClient { id, config, stream })
    }

    pub fn run_listener(&self) -> Result<(), Error> {
        //let mut stream_cpy = self.stream.try_clone()?;
        //let mut counter = 0;

        let mut stream_cpy = match self.stream.try_clone() {
            Ok(client_stream) => client_stream,
            Err(e) => {
                eprintln!("{}", e);
                return Err(e);
            }
        };

        println!("Listener initialized");
        /*
        // falta un listener de paquetes, que use la construccion del header ...
        while let Ok(header) = PacketFixedHeader::read_from(&mut &self.stream) {
            println!("listening packages");
            match &self.messages_handler(&mut stream_cpy, header) {
                Ok(_) => continue,
                Err(e) => {
                    eprintln!("{}", e);
                    continue
                },
            }
            //counter = 0;
        };
        */
        let mut counter = 0;
        let l_file = RwLock::new(&self.stream);
        // let mut l_read_file ;
        loop {
            // lockeo de lectura:
            match l_file.read() {
                Ok(lread_file) => {
                    match PacketFixedHeader::read_from(&mut lread_file.clone()) {
                        Ok(header) => {
                            println!("listening packages");
                            self.messages_handler(&mut stream_cpy, header)?;
                            counter = 0;
                        }
                        Err(e) => {
                            eprintln!("listening package error: {}", e);
                            thread::sleep(std::time::Duration::from_secs(10));
                            counter += 10;
                            if let Some(expiry_interval) =
                                self.config.connect_properties.session_expiry_interval
                            {
                                if expiry_interval == 0 {
                                    continue;
                                }
                                if counter > expiry_interval {
                                    break;
                                }
                            }
                            continue;
                        }
                    };
                }
                Err(_) => todo!(),
            }
        }

        //Err(Error::new(std::io::ErrorKind::Other, "Session expired"))
        Ok(())
    }

    pub fn messages_handler(
        &self,
        mut stream: &mut TcpStream,
        fixed_header: PacketFixedHeader,
    ) -> Result<(), Error> {
        println!("Package incoming 1");
        let packet_recived = match get_packet(
            &mut stream,
            fixed_header.get_package_type(),
            fixed_header.remaining_length,
        ) {
            Ok(received) => received,
            Err(e) => {
                eprintln!("Error at reading package");
                return Err(e);
            }
        };

        println!("Package incoming 2");
        match packet_recived {
            PacketReceived::Publish(_publish) => {
                /*
                // para logear, el cliente debe pasar el mensaje al servidor
                MqttActions::ClientReceive(self.id.clone(), publish.properties.topic_name.clone())
                    .register_action();
                */
                println!("Package received")
            }
            _ => return Err(Error::new(std::io::ErrorKind::Other, "Paquete desconocido")),
        }

        Ok(())
    }

    pub fn publish() {
        todo!()
    }

    pub fn subscribe() {
        todo!()
    }

    pub fn unsubscribe() {
        todo!()
    }

    pub fn disconnect() {
        todo!()
    }

    pub fn pin_request() {
        todo!()
    }
}
