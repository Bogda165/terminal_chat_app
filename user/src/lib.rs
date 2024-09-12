use custom_errors::CustomError;
use Commands::Command;
#[derive(Debug)]
pub struct User {
    addr_recv: (String, u16),
    addr_send: (String, u16),
}

impl User {
    pub fn new() -> Self {
        User {
            addr_send: ("0.0.0.0".to_string(), 0),
            addr_recv: ("0.0.0.0".to_string(), 0),
        }
    }

    pub fn new_from(addr_recv: (String, u16), addr_send: (String, u16)) -> Self {
        User {
            addr_recv,
            addr_send,
        }
    }

    pub fn from_command(cmd: Command) -> Result<Self, CustomError> {
        match cmd {
            Command::Connect {addr_recv, addr_send, password: _,}=> {
                Ok(User::new_from(addr_send, addr_recv))
            },
            _ => {
                Err(CustomError::InvalidCommand)
            }
        }
    }

    pub fn to_command(&self, cmd: Command) -> Result<Command, CustomError> {
        match cmd {
            Command::Connect { .. } => {
                Ok(Command::Connect {
                    addr_recv: self.addr_recv.clone(),
                    addr_send: self.addr_send.clone(),
                    password: false,
                })
            },
            _ => {Err(CustomError::FailedConverting)}
        }
    }
}