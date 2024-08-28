use tokio::sync::RwLock;
use user::User;
use ::Peer::Server;
use Peer::peer::Peer;
use std::sync::Arc;
use Commands::Command;

pub struct Client
{
    pub server: Server,
    main_server: (String, u16, bool),
    users_chats: Arc<RwLock<Vec<User>>>
}

impl Client
{
    pub fn get_user(&self) -> User {
        let addr = self.server.get_addr();
        let port = self.server.get_recv_port();
        User::new_from(addr.clone(), port)
    }

    fn handler (cmd: Command) {
        match cmd {
            Connect => {
                println!("{:?}", Connect);
            },
            Disconnect => {
                println!("Disconnect");
            },
        }
    }

    pub async fn new(addr: String, recv_port: u16, send_port: u16) -> Self {
        let mut server = Server::new(addr, recv_port, send_port).await;
        let users: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(Vec::new()));

        Client {
            server,
            users_chats: users,
            main_server: ("".to_string(), 0, false),
        }
    }

    pub fn set_main_server(&mut self, _main_server: (String, u16, bool)) {
        self.main_server = _main_server;
    }
    // Results will be handled by customed function "handler", passed to peer::run
    pub async fn try_connect(&self) {

        let self_user = self.get_user();
        let command = self_user.to_command(Command::Connect {
            addr: "".to_string(),
            port: 0,
            password: false,
        }).unwrap();
        let command = self_user.to_command(command).unwrap();
        let command = bincode::serialize(&command).unwrap();

        self.server.send(self.main_server.0.clone(), self.main_server.1, command.as_slice()).await;
    }

}