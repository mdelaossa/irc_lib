use crate::message::{Command, Param, IrcMessage};
use crate::connection::Connection;
use crate::{connection::ConnectionNegotiator, Config};

use std::{collections::HashMap, sync::{
        mpsc::{self, Sender},
        Arc, Mutex,
    }, thread};


use super::channel::Channel;
use super::user::User;
use super::error::{Error, Result};
use super::client::Client;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: HashMap<String, Channel>,
    config: Config,
    connection: Arc<Mutex<Connection>>,
    sender: Option<Sender<IrcMessage>>
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

    pub fn send_message(&self, message: IrcMessage) -> Result<()> {
        if let Some(sender) = &self.sender {
            sender.send(message).map_err(|r| Error::WriteError(self.address.clone(), r.to_string()))
        } else {
            Err(Error::WriteError(self.address.clone(), "Not connected".to_string()))
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

            let mut negotiator = ConnectionNegotiator::new(&self.config);

            loop {
                let mut conn = match connection.lock() {
                    Ok(conn) => conn,
                    Err(_) => continue,
                };

                for outgoing in thread_rcv.try_iter() {
                    let _ = conn.send_message(&outgoing.to_string());
                }

                match conn.read() {
                    Ok(Some(message)) => {
                        println!("RECEIVED: {:?}", message);

                        match &message {
                            IrcMessage { command: Command::Numeric(353), params, ..} => self.parse_users(params),
                            IrcMessage { command: Command::Ping, .. } => Self::ping_response(&mut conn, &message),
                            IrcMessage { command: Command::PrivMsg, params, .. } => {
                                for param in params {
                                    if let Param::Message(message) = param {
                                        if message.contains('\u{1}') {
                                            // CTCP message
                                            Self::version_response(&mut conn, message)
                                        }
                                    }
                                }
                            },
                            IrcMessage { command: Command::Version, .. } => conn.send_message("VERSION 123").unwrap(),
                            _ => (),
                            
                        }
                        thread_snd.send(message.clone()).ok();
                        for plugin in self.config.plugins.iter() {
                            plugin.message(&self, &message)
                        }

                    },
                    Ok(None) => {
                        if let Some(message) = negotiator.next() {
                            let _ = conn.send_message(&message);
                        }
                    }
                    Err(e) => {
                        println!("Error reading from connection: {:?}", e);
                        break;
                    },                
                }
            }
        });

        Client {
            thread: Some(thread),
            rcv_channel: Some(rcv_channel),
            snd_channel: Some(snd_channel),
        }
    }

    // This is a 353 message we need to parse
    fn parse_users(&mut self, params: &[Param]) {
        // 2nd param is the channel name, 3rd and onwards are the users
        let channel_name = params[2].to_string();
        let channel = self.channels.entry(channel_name.to_string()).or_insert(Channel::new(&channel_name));
        println!("Channel: {:?}", channel);
        for param in params[3..].iter() {
            if let Param::Unknown(user) = param {
                let user = User::new(user);
                channel.users.insert(user.nick.clone(), user);
            }
        }
        println!("Channel: {:?}", channel);
    }

    fn ping_response(connection: &mut Connection, message: &IrcMessage) {
        let msg = message.params.iter().find_map(|param| {
            if let Param::Message(ref msg) = param {
                Some(msg)
            } else {
                None
            }
        });
    
        if let Some(msg) = msg {
            connection
            .send_message(&format!(
                "PONG :{}",
                msg
            ))
            .unwrap()
        }
    }
    
    fn version_response(connection: &mut Connection, message: &str) {
        connection
            .send_message(&format!(
                "NOTICE :{} PRIVMSG :\u{1}VERSION 1\u{1}",
                message
            ))
            .unwrap();
    }
}
