use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = TcpStream::connect("irc.subluminal.net:6667")?;

    let mut buff = [0; 512];

    let mut negotiator = Negotiator::new();

    loop {
        match connection.read(&mut buff) {
        Ok(size) => {
            if size == 0 {
                continue;
            }
            println!("{:?}", std::str::from_utf8(&buff[..size])?);

            if std::str::from_utf8(&buff[0..size])?.contains("PING") {
                send_message(&mut connection, b"PONG")?;
            } else if std::str::from_utf8(&buff[0..size])?.contains("VERSION") {
                send_message(&mut connection, b"VERSION 123")?;
            } else if negotiator.negotiate {
                negotiator.negotiate(&mut connection)?;
            }else {
                send_message(&mut connection, b"hi")?;
            }
        },
        Err(err) => return Err(Box::new(err))
        }
    }
 }

 struct Negotiator {
     round: u8,
     negotiate: bool
 }

 impl Negotiator {
    fn new() -> Negotiator {
        Negotiator {
            round: 0,
            negotiate: true
        }
    }

     fn negotiate(&mut self, server: &mut TcpStream) -> Result<(), Box<dyn std::error::Error>> {
         match self.round {
             1 => {
                send_message(server, b"NICK testing_a_rusty_thing")?;
                self.round += 1;
             },
             2 => {
                send_message(server, b"USERNAME rusty 0 * None")?;
                self.round += 1;
                self.negotiate = false;
             },
             _ => self.round += 1
         }
         Ok(())
     }
 }

 fn send_message(server: &mut TcpStream, message: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
     let message = &[message, b"\r\n"].concat();
     println!("SENDING: {:?}", std::str::from_utf8(message)?);
     server.write(message)?;
     Ok(())
 }

