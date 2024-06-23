use std::{collections::HashMap, net::TcpStream};

#[derive(Default)]
pub struct ServerNetwork {
    pub connections: HashMap<String, TcpStream>,
}

impl Clone for ServerNetwork {
    fn clone(&self) -> Self {
        let mut connections = HashMap::new();
        for (key, value) in self.connections.iter() {
            connections.insert(key.clone(), value.try_clone().unwrap());
        }
        ServerNetwork { connections }
    }
}
