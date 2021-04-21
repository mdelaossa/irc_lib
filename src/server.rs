use irc_rust::Message;

mod channel;
mod user;

use std::{sync::{Arc, Mutex, mpsc::{self, Receiver, Sender}}, thread, thread::{JoinHandle}};

use crate::{Config, connection::negotiator::Negotiator};
use crate::connection::Connection;

use channel::Channel;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: Vec<Channel>,
    config: Config,
    connection: Arc<Mutex<Connection>>
}

#[derive(Debug)]
pub struct Client {
    thread: Option<JoinHandle<()>>,
    snd_channel: Sender<Message>,
    rcv_channel: Receiver<Message>
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
            channels: Vec::new(),
            connection: Arc::new(Mutex::new(Connection::new())),
            config
        }
    }

    pub fn run(self) -> Client {
        self.connect()
    }

    pub fn send_message(&self, message: &str) {
        if let Ok(mut connection) = self.connection.lock() {
            connection.send_message(message).ok();
        }
    }

    fn connect(self) -> Client {
        let connection = self.connection.clone();
        let (thread_snd, rcv_channel) = mpsc::channel::<Message>();
        let (snd_channel, thread_rcv) = mpsc::channel::<Message>();

        let thread = thread::spawn(move || {           
            connection.lock().unwrap().connect(self.address.to_owned()).unwrap();

            let mut negotiator = Negotiator::new(&self.config);

            loop {
                let mut connection = connection.lock().unwrap();

                for outgoing in thread_rcv.try_iter() {
                    connection.send_message(&outgoing.to_string()).unwrap();
                }

                let message = connection.read().unwrap();

                match message {
                    Some(message) => {
                        println!("RECEIVED: {:?}", message);
                
                        if message.command().contains("PING") {
                            connection.send_message(&format!("PONG :{}", message.params().unwrap().trailing().unwrap())).unwrap();
                        } else if message.command().contains("VERSION") {
                            connection.send_message("VERSION 123").unwrap();
                        } else if message.params().unwrap().to_string().contains("\u{1}") { // CTCP message
                            // Parse here, for now only return version.
                            connection.send_message(&format!("NOTICE :{} PRIVMSG :\u{1}VERSION 1\u{1}", message.prefix().unwrap().unwrap())).unwrap();
                        } else {
                            drop(connection); // Unlock mutex on Connection
                            thread_snd.send(message.clone()).ok();
                            for plugin in self.config.plugins.iter() {
                                plugin.message(&self, &message)
                            }
                        }
                    },
                    None => {
                        match negotiator.next() {
                            Some(message) => connection.send_message(&message).unwrap(),
                            None => continue
                        };
                    }
                };
            }
        });

        Client {
            thread: Some(thread),
            rcv_channel,
            snd_channel
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().expect("Critical error with IRC Client. Aborting");
        }
    }
}

impl Client {
    pub fn channels(&self) -> (&Sender<Message>, &Receiver<Message>) {
        (&self.snd_channel, &self.rcv_channel)
    }
}
