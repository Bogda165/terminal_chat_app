
#[derive(Debug)]
pub struct ServerHeader {
    pub id: Option<u16>,
    pub recv: (String, u16),
    pub send: (String, u16),
}

impl ServerHeader {
    pub fn new() -> Self {
        ServerHeader {
            recv: ("".to_string(), 0),
            send: ("".to_string(), 1),
            id: None
        }
    }
}