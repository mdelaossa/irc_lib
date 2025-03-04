use crate::message::{IrcMessage, Message};
use crate::connection::Connection;
use crate::{connection::negotiator::Negotiator, Config};

use std::{collections::HashMap, sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    }, thread::{self, JoinHandle}};


use err_derive::Error;

use channel::Channel;

use self::user::User;

pub mod channel;
pub mod user;


#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: HashMap<String, Channel>,
    config: Config,
    connection: Arc<Mutex<Connection>>,
    sender: Option<Sender<IrcMessage>>
}

#[derive(Debug)]
pub struct Client {
    thread: Option<JoinHandle<()>>,
    snd_channel: Option<Sender<IrcMessage>>,
    rcv_channel: Option<Receiver<IrcMessage>>,
}

#[derive(Debug, Error)]
pub enum IrcError {
    #[error(display="failed to connect to {:?}", _0)]
    ConnectionError(String),
    #[error(display="failed to read from {:?}", _0)]
    ReadError(String),
    #[error(display="failed to write to {:?}. Reason: {:?}", _0, _1)]
    WriteError(String, String),
}


impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            address: config.server.clone(),
            channels: config.channels.clone(),
            connection: Arc::new(Mutex::new(Connection::new())),
            sender: None,
            config,
        }
    }

    pub fn run(self) -> Client {
        self.connect()
    }

    pub fn send_message(&self, message: IrcMessage) -> Result<(), IrcError> {
        if let Some(sender) = &self.sender {
            sender.send(message).map_err(|r| IrcError::WriteError(self.address.clone(), r.to_string()))
        } else {
            Err(IrcError::WriteError(self.address.clone(), "Not connected".to_string()))
        }
    }

    fn connect(mut self) -> Client {
        let connection = self.connection.clone();
        let (thread_snd, rcv_channel) = mpsc::channel::<IrcMessage>();
        let (snd_channel, thread_rcv) = mpsc::channel::<IrcMessage>();
        self.sender = Some(snd_channel.clone());

        let thread = thread::spawn(move || {
            if let Ok(mut conn) = connection.lock() {
                if conn.connect(self.address.clone()).is_err() {
                    return;
                }
            }

            let mut negotiator = Negotiator::new(&self.config);

            loop {
                let mut conn = match connection.lock() {
                    Ok(conn) => conn,
                    Err(_) => continue,
                };

                for outgoing in thread_rcv.try_iter() {
                    let _ = conn.send_message(&outgoing.to_string());
                }

                if let Ok(Some(message)) = conn.read(self.channels.clone()) {
                    println!("RECEIVED: {:?}", message);

                    match &message {
                        IrcMessage::Numeric(353, message) => self.parse_users(message),
                        IrcMessage::PING(message) => ping_response(&mut conn, message),
                        IrcMessage::PRIVMSG(message)
                            if message.params().map(|p| p.to_string().contains('\u{1}')).unwrap_or(false) =>
                        {
                            // CTCP message
                            // TODO: Parse here, for now only return version.
                            version_response(&mut conn, message)
                        }
                        IrcMessage::VERSION(_) => conn.send_message("VERSION 123").unwrap(),
                        _ => (),
                        
                    }
                    thread_snd.send(message.clone()).ok();
                    for plugin in self.config.plugins.iter() {
                        plugin.message(&self, &message)
                    }

                } else if let Some(message) = negotiator.next() {
                    let _ = conn.send_message(&message);
                };
            }
        });

        Client {
            thread: Some(thread),
            rcv_channel: Some(rcv_channel),
            snd_channel: Some(snd_channel),
        }
    }

    fn parse_users(&mut self, message: &Message) {
        if let Some(params) = message.params() {
            println!("Parsing users from {}", params);
            let chan_name = params.iter().last().unwrap();
            let users = params.trailing().unwrap().split_whitespace();

            let channel = self.channels.entry(chan_name.to_string()).or_insert(Channel::new(chan_name));

            for user in users {
                let user = User::new(user);
                channel.users.insert(user.nick.clone(), user);
            }
            println!("Channel: {:?}", channel);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        // If we're here, no one is using our channels.
        // Let's drop them so that Server doesn't grow its channels' buffer into the stratosphere
        drop(self.snd_channel.take());
        drop(self.rcv_channel.take());

        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .expect("Critical error with IRC Client. Aborting");
        }
    }
}

impl Client {
    pub fn channels(&self) -> (&Sender<IrcMessage>, &Receiver<IrcMessage>) {
        (
            self.snd_channel.as_ref().unwrap(),
            self.rcv_channel.as_ref().unwrap(),
        )
    }
}

fn ping_response(connection: &mut Connection, message: &Message) {
    connection
        .send_message(&format!(
            "PONG :{}",
            message.params().unwrap().trailing().unwrap()
        ))
        .unwrap()
}

fn version_response(connection: &mut Connection, message: &Message) {
    connection
        .send_message(&format!(
            "NOTICE :{} PRIVMSG :\u{1}VERSION 1\u{1}",
            message.prefix().unwrap().unwrap()
        ))
        .unwrap();
}
