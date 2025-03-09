use crate::connection::IrcConnection;
use crate::message::{Command, IrcMessage, Param};
use crate::{Config, connection::ConnectionNegotiator};

use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread,
};

use super::channel::Channel;
use super::client::Client;
use super::error::{Error, Result};
use super::user::User;

#[derive(Debug)]
pub struct Server {
    pub address: String,
    pub channels: HashMap<String, Channel>,
    config: Config,
    connection: Arc<Mutex<Box<dyn IrcConnection>>>,
    sender: Option<Sender<IrcMessage>>,
}

impl Server {
    pub fn new(config: Config, connection: Box<dyn IrcConnection>) -> Self {
        Self {
            address: config.server.clone(),
            channels: config.channels.clone(),
            connection: Arc::new(Mutex::new(connection)),
            sender: None,
            config,
        }
    }

    pub fn run(self) -> Client {
        self.connect()
    }

    pub fn send_message(&self, message: IrcMessage) -> Result<()> {
        if let Some(sender) = &self.sender {
            sender
                .send(message)
                .map_err(|r| Error::Write(self.address.clone(), r.to_string()))
        } else {
            Err(Error::Write(
                self.address.clone(),
                "Not connected".to_string(),
            ))
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
                            IrcMessage {
                                command: Command::Numeric(353),
                                params,
                                ..
                            } => self.parse_users(params),
                            IrcMessage {
                                command: Command::Ping,
                                ..
                            } => Self::ping_response(&mut **conn, &message),
                            IrcMessage {
                                command: Command::PrivMsg,
                                params,
                                ..
                            } => {
                                for param in params {
                                    if let Param::Message(message) = param {
                                        if message.contains('\u{1}') {
                                            // CTCP message
                                            Self::version_response(&mut **conn, message)
                                        }
                                    }
                                }
                            }
                            IrcMessage {
                                command: Command::Version,
                                ..
                            } => conn.send_message("VERSION 123").unwrap(),
                            _ => (),
                        }
                        thread_snd.send(message.clone()).ok();
                        for plugin in self.config.plugins.iter() {
                            plugin.message(&self, &message)
                        }
                    }
                    Ok(None) => {
                        if let Some(message) = negotiator.next() {
                            let _ = conn.send_message(&message);
                        }
                    }
                    Err(e) => {
                        println!("Error reading from connection: {:?}", e);
                        break;
                    }
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
        let channel = self
            .channels
            .entry(channel_name.to_string())
            .or_insert(Channel::new(&channel_name));
        println!("Channel: {:?}", channel);
        for param in params[3..].iter() {
            if let Param::Unknown(user) = param {
                let user = User::new(user);
                channel.users.insert(user.nick.clone(), user);
            }
        }
        println!("Channel: {:?}", channel);
    }

    fn ping_response(connection: &mut dyn IrcConnection, message: &IrcMessage) {
        let msg = message.params.iter().find_map(|param| {
            if let Param::Message(msg) = param {
                Some(msg)
            } else {
                None
            }
        });

        if let Some(msg) = msg {
            connection.send_message(&format!("PONG :{}", msg)).unwrap()
        }
    }

    fn version_response(connection: &mut dyn IrcConnection, message: &str) {
        connection
            .send_message(&format!("NOTICE :{} PRIVMSG :\u{1}VERSION 1\u{1}", message))
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::MockIrcConnection;
    use crate::message::{Command, IrcMessage, Param};
    use std::time::Duration;

    #[test]
    fn test_ping_response() {
        let mut mock_conn = MockIrcConnection::new();
        mock_conn
            .expect_send_message()
            .with(mockall::predicate::eq("PONG :12345"))
            .times(1)
            .returning(|_| Ok(()));

        let message = IrcMessage {
            prefix: None,
            command: Command::Ping,
            params: vec![Param::Message("12345".to_string())],
        };

        Server::ping_response(&mut mock_conn, &message);
    }

    #[test]
    fn test_version_response() {
        let mut mock_conn = MockIrcConnection::new();
        mock_conn
            .expect_send_message()
            .with(mockall::predicate::eq(
                "NOTICE :test_user PRIVMSG :\u{1}VERSION 1\u{1}",
            ))
            .times(1)
            .returning(|_| Ok(()));

        let message = "test_user";

        Server::version_response(&mut mock_conn, message);
    }

    #[test]
    fn test_parse_users() {
        let config = Config {
            nick: "test".to_string(),
            user: "test".to_string(),
            server: "localhost".to_string(),
            channels: HashMap::new(),
            plugins: vec![],
        };
        let mock_conn = MockIrcConnection::new();
        let mut server = Server::new(config, Box::new(mock_conn));

        let params = vec![
            Param::Unknown("".to_string()),
            Param::Unknown("".to_string()),
            Param::Channel("#test".to_string()),
            Param::Unknown("user1".to_string()),
            Param::Unknown("user2".to_string()),
        ];

        server.parse_users(&params);

        let channel = server.channels.get("#test").unwrap();
        assert!(channel.users.contains_key("user1"));
        assert!(channel.users.contains_key("user2"));
    }

    #[test]
    fn test_connect_loop() {
        let config = Config {
            nick: "test".to_string(),
            user: "test".to_string(),
            server: "localhost".to_string(),
            channels: HashMap::new(),
            plugins: vec![],
        };

        let mut mock_conn = MockIrcConnection::new();
        mock_conn
            .expect_connect()
            .with(mockall::predicate::eq("localhost".to_string()))
            .times(1)
            .returning(|_| Ok(()));

        mock_conn.expect_read().times(1).returning(|| {
            Ok(Some(IrcMessage {
                prefix: None,
                command: Command::Ping,
                params: vec![Param::Message("12345".to_string())],
            }))
        });

        mock_conn
            .expect_read()
            .times(1)
            .returning(|| Err(crate::connection::error::Error::ConnectionClosed));

        mock_conn
            .expect_send_message()
            .with(mockall::predicate::eq("PONG :12345"))
            .times(1)
            .returning(|_| Ok(()));

        let server = Server::new(config, Box::new(mock_conn));
        let client = server.run();

        // Give the thread some time to process
        thread::sleep(Duration::from_millis(100));

        // Ensure the client thread is still running
        assert!(client.thread.is_some());
    }
}
