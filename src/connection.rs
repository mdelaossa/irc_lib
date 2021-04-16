pub mod negotiator;

use std::io::prelude::*;
use std::net::TcpStream;

use super::message::IrcMessage;

pub(crate) struct Connection {
    socket: Option<TcpStream>,
    buffer: super::RawIrcMessage
}

impl Connection {
    fn not_connected() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotConnected, "Not Connected")
    }

    pub(crate) fn new() -> Connection {
        Connection {
            socket: None,
            buffer: [0; 512]
        }
    }

    pub(crate) fn connect(&mut self, address: &str) -> Result<(), std::io::Error> {
        match TcpStream::connect(address) {
            Ok(stream) => {
                self.socket = Some(stream);
                Ok(())
            },
            Err(r) => {
                self.socket = None;
                Err(r)
            }
        }
    }

    pub(crate) fn send_message(&mut self, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.socket {
            Some(stream) => {
                let message = &[message, b"\r\n"].concat();
                println!("SENDING: {:?}", std::str::from_utf8(message)?);
                stream.write_all(message)?;
                Ok(())
            },
            _ => Err(Box::new(Connection::not_connected()))
        }
    }

    pub(crate) fn read(&mut self) -> Result<IrcMessage, std::io::Error>{
        match &mut self.socket {
            Some(stream) => {
                // Get rid of any old messages in the buffer
                self.buffer = [0;512];
                let size = stream.read(&mut self.buffer)?;
                Ok(IrcMessage{
                    size,
                    text: &self.buffer
                })
            },
            _ => Err(Connection::not_connected())
        }
    }
}
