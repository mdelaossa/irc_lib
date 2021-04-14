pub mod connection;
pub mod message;

// IRC protocol denotes 512 bytes as the max message length
type RawIRCMessage = [u8; 512];

pub struct Client {
    pub server: String,
    client : connection::Connection
}

impl Client {
    pub fn new(server: &str) -> Client {
        Client {
            server: server.to_owned(),
            client: connection::Connection::new()
        }
    }

    pub fn connect(&mut self) -> Result<(), std::io::Error> {
        self.client.connect(&self.server)
    }

    pub fn send_message(&mut self, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.client.send_message(message)
    }

    pub fn read(&mut self) -> Result<message::IRCMessage, std::io::Error> {
        self.client.read()
     }
}