use std::{io::{BufRead, BufReader, Write}, time::Duration};
use std::net::TcpStream;

use crate::message::IrcMessage;
use super::error::{Result, Error};

#[derive(Debug)]
pub(crate) struct Connection {
    socket: Option<BufReader<TcpStream>>,
    buffer: String
}

impl Connection {
    pub(crate) fn new() -> Connection {
        Connection {
            socket: None,
            buffer: String::new()        
        }
    }

    pub(crate) fn connect(&mut self, address: String) -> Result<()> {
        let stream = TcpStream::connect(address)?;
        stream.set_read_timeout(Some(Duration::from_millis(1500)))?;
        self.socket = Some(BufReader::new(stream));
        Ok(())
    }

    pub(crate) fn send_message(&mut self, message: &str) -> Result<()> {
        match &mut self.socket {
            Some(stream) => {
                let bytes = &[message.as_bytes(), b"\r\n"].concat();
                println!("SENDING: {:?}", message);
                stream.get_ref().write_all(bytes)?;
                Ok(())
            },
            _ => Err(Error::NotConnected)
        }
    }

    pub(crate) fn read(&mut self) -> Result<Option<IrcMessage>>{
        match &mut self.socket {
            Some(stream) => {
                // Get rid of any old messages in the buffer
                self.buffer.clear();

                match stream.read_line(&mut self.buffer) {
                    Ok(0) => {
                        // Connection closed
                        self.socket = None;
                        Err(Error::ConnectionClosed)
                    }
                    Ok(_) => {
                        let msg: IrcMessage = self.buffer.as_str().parse()?;
                        Ok(Some(msg))
                    }
                    Err(e) => {
                        match e.kind() {
                            std::io::ErrorKind::WouldBlock => {
                                // Timed out, return so we can do stuff
                                Ok(None)
                            },
                            _ => Err(e.into())
                        }
                    }
                }
            },
            None => Err(Error::NotConnected)
        }
    }
}
