pub mod negotiator;

use std::io::prelude::*;
use std::net::TcpStream;

pub struct Connection {
    socket: Option<TcpStream>
}

impl Connection {
    fn not_connected() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotConnected, "Not Connected")
    }

    pub fn new() -> Connection {
        Connection {
            socket: None
        }
    }

    pub fn connect(&mut self, address: &str) -> Result<(), std::io::Error> {
        match TcpStream::connect(address) {
            Ok(stream) => {
                self.socket = Some(stream);
                return Ok(());
            },
            Err(r) => {
                self.socket = None;
                return Err(r);
            }
        }
    }

    pub fn send_message(&mut self, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.socket {
            Some(stream) => {
                let message = &[message, b"\r\n"].concat();
                println!("SENDING: {:?}", std::str::from_utf8(message)?);
                stream.write(message)?;
                Ok(())
            },
            _ => Err(Box::new(Connection::not_connected()))
        }
    }

    pub fn read(&mut self, buff: &mut [u8]) -> Result<usize, std::io::Error>{
        match &mut self.socket {
            Some(stream) => stream.read(buff),
            _ => Err(Connection::not_connected())
        }
    }
}
