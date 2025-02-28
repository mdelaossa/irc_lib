use crate::message::{IrcMessage, Message};

pub mod channel;
pub mod user;

use std::{collections::HashMap, sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    }, thread::{self, JoinHandle}};

use crate::connection::Connection;
use crate::{connection::negotiator::Negotiator, Config};

use channel::Channel;

use self::user::User;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: HashMap<String, Channel>,
    config: Config,
    connection: Arc<Mutex<Connection>>,
}

#[derive(Debug)]
pub struct Client {
    thread: Option<JoinHandle<()>>,
    snd_channel: Option<Sender<IrcMessage>>,
    rcv_channel: Option<Receiver<IrcMessage>>,
}

pub trait IrcError {}

#[derive(Debug, Clone)]
struct ConnectionError<'a> {
    server_address: &'a str,
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

#[derive(Debug, Clone)]
pub struct WriteError;

impl IrcError for WriteError {}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "failure to write to server")
    }
}

impl Server {
    pub fn new(config: Config) -> Self {
        Self {
            address: config.server.clone(),
            channels: config.channels.clone(),
            connection: Arc::new(Mutex::new(Connection::new())),
            config,
        }
    }

    pub fn run(self) -> Client {
        self.connect()
    }

    pub fn send_message(&self, message: &str) -> Result<(), WriteError> {
        self.connection
            .lock()
            .map_err(|_| WriteError)?
            .send_message(message)
            .map_err(|_| WriteError)
    }

    fn connect(mut self) -> Client {
        let connection = self.connection.clone();
        let (thread_snd, rcv_channel) = mpsc::channel::<IrcMessage>();
        let (snd_channel, thread_rcv) = mpsc::channel::<IrcMessage>();

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
                    drop(conn); // Unlock mutex on Connection - needed so plugins can send messages
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
