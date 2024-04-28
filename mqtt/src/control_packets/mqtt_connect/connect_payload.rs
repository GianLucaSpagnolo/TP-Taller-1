pub struct ConnectPayload {
    client_id: String,
}

impl ConnectPayload {
    
    pub fn lenght(&self) -> usize {
        todo!()
    }

    pub fn new(client_id: String) -> Self {
        ConnectPayload {
            client_id
        }
    }
}