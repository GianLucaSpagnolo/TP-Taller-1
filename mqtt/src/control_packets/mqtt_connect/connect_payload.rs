pub struct _ConnectPayload {
    client_id: String,
}

impl _ConnectPayload {
    pub fn _lenght(&self) -> usize {
        todo!()
    }

    pub fn _new(client_id: String) -> Self {
        _ConnectPayload { client_id }
    }
}
