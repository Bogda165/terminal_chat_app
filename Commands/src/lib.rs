use serde::{Serialize, Deserialize};
use custom_errors::CustomError;

#[derive(
Serialize, Deserialize, Debug
)]
pub enum MessageD {
    Text {message: String},
    File {path: String},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    Connect {addr_send: (String, u16), addr_recv: (String, u16), password: bool, add_info: String},
    Disconnect {addr: String, port: u16},
    // id of a user, a message was send by
    Message {id: i32, data: MessageD}
}

impl Command {
    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize command")
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self, CustomError> {
        match serde_json::from_slice(data.as_slice()) {
            Ok(cmd) => Ok(cmd),
            Err(_) => {Err(CustomError::FailedConverting)}
        }
    }
}