mod channel;
mod user;

use std::thread;

use crate::Config;
use crate::connection::Connection;
// use crate::message::IrcMessage;

use channel::Channel;
// use user::User;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: Vec<Channel>,
    config: Config,
    thread: Option<thread::JoinHandle<()>>
}

pub trait IrcError {}

#[derive(Debug, Clone)]
struct ConnectionError<'a> {
    server_address: &'a str
}

impl<'a> IrcError for ConnectionError<'a> {}

impl<'a> std::fmt::Display for ConnectionError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failure to connect to {}", self.server_address)
    }
}

#[derive(Debug, Clone)]
struct ReadError;

impl IrcError for ReadError {}

impl std::fmt::Display for ReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failure to read from server")
    } 
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            address: config.server.to_owned(),
            thread: None,
            channels: Vec::new(),
            config
        }
    }

    pub fn run(self) {
        self.connect();
    }

    fn connect(mut self) {
        let address = self.address.to_owned();
        let config = self.config;

        self.thread = Some(std::thread::spawn(move || {
            let mut connection = Connection::new();

            connection.connect(address).unwrap();

            let mut negotiator = crate::connection::negotiator::Negotiator::new();

            loop {
                let message = connection.read().unwrap();

                println!("RECEIVED: {:?}", message.text);
                
                if message.text.contains("PING") {
                    connection.send_message("PONG").unwrap();
                } else if message.text.contains("VERSION") {
                    connection.send_message("VERSION 123").unwrap();
                } else if let Some(message) = negotiator.next() {
                    connection.send_message(message).unwrap();
                }else {
                    for plugin in config.plugins.iter() {
                        plugin.message(&message)
                    }
                }
            }
        }));

        self.thread.unwrap().join().unwrap();
    }
}