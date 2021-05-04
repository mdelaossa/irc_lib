pub mod negotiator;

use std::{io::{BufReader, prelude::*}, time::Duration};
use std::net::TcpStream;

use crate::message::IrcMessage;

#[derive(Debug)]
pub(crate) struct Connection {
    socket: Option<BufReader<TcpStream>>,
    buffer: String
}

impl Connection {
    fn not_connected() -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotConnected, "Not Connected")
    }

    pub(crate) fn new() -> Connection {
        Connection {
            socket: None,
            buffer: String::new()        }
    }

    pub(crate) fn connect(&mut self, address: String) -> Result<(), std::io::Error> {
        match TcpStream::connect(address) {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_millis(1500))).unwrap();
                self.socket = Some(BufReader::new(stream));
                Ok(())
            },
            Err(r) => {
                self.socket = None;
                Err(r)
            }
        }
    }

    pub(crate) fn send_message(&mut self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        match &mut self.socket {
            Some(stream) => {
                let bytes = &[message.as_bytes(), b"\r\n"].concat();
                println!("SENDING: {:?}", message);
                stream.get_ref().write_all(bytes)?;
                Ok(())
            },
            _ => Err(Box::new(Connection::not_connected()))
        }
    }

    pub(crate) fn read(&mut self) -> Result<Option<IrcMessage>, std::io::Error>{
        match &mut self.socket {
            Some(stream) => {
                // Get rid of any old messages in the buffer
                self.buffer = String::new();

                match stream.read_line(&mut self.buffer) {
                    Ok(_) => {
                        Ok(Some(IrcMessage::from(self.buffer.as_str()).unwrap()))
                    },
                    Err(e) => {
                        match e.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                // Timed out, return so we can do stuff
                                Ok(None)
                            },
                            _ => Err(e)
                        }
                    }
                }
            },
            _ => Err(Connection::not_connected())
        }
    }
}
