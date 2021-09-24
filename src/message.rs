use std::{collections::HashMap, ops::Deref};

use crate::server::channel::Channel;

#[derive(Debug, Clone)]
pub struct Message {
    raw: irc_rust::Message,
    pub channel: Option<Channel>
}

#[derive(Debug, Clone)]
pub enum IrcMessage {
    Numeric(u32, Message),
    CAP(Message),
    JOIN(Message),
    MODE(Message),
    NOTICE(Message),
    PING(Message),
    PONG(Message),
    PRIVMSG(Message),
    VERSION(Message),
    QUIT(Message),
}

impl Message {
    fn from(str: &str) -> Self {
        let msg = irc_rust::Message::from(str);

        Self { raw: msg, channel: None }
    }

    fn from_str(str: &str) -> Self {
        let msg = irc_rust::Message::from(str);
 
        Self { raw: msg, channel: None }
    }
}

impl Deref for Message {
    type Target = irc_rust::Message;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl IrcMessage {
    fn parse_command(message: Message) -> Result<Self, String> {
        let command = message.command();

        match command {
            "CAP" => Ok(IrcMessage::CAP(message)),
            "JOIN" => Ok(IrcMessage::JOIN(message)),
            "MODE" => Ok(IrcMessage::MODE(message)),
            "NOTICE" => Ok(IrcMessage::NOTICE(message)),
            "PING" => Ok(IrcMessage::PING(message)),
            "PONG" => Ok(IrcMessage::PONG(message)),
            "PRIVMSG" => Ok(IrcMessage::PRIVMSG(message)),
            "VERSION" => Ok(IrcMessage::VERSION(message)),
            "QUIT" => Ok(IrcMessage::QUIT(message)),
            _ => {
                if let Ok(num) = command.parse::<u32>() {
                    Ok(IrcMessage::Numeric(num, message))
                } else {
                    Err(format!("Failed to convert {} into IrcMessage", command))
                }
            }
        }  
    }

    pub(crate) fn from_raw(s: &str, channels: HashMap<String, Channel>) -> Result<Self, String> {
        let message = Message::from(s);

        let parsed_message = Self::parse_command(message);

        parsed_message.map(|message| {
            if let IrcMessage::PRIVMSG(mut message) = message {
                let channel = channels.get(message.params().unwrap().iter().next().unwrap_or("")).cloned();
                message.channel = channel;
                
                return IrcMessage::PRIVMSG(message)
            } 
            message
        })
    }

    pub fn from(s: &str) -> Result<Self, String> {
        let message = Message::from_str(s);

        Self::parse_command(message)
    }
}

impl Deref for IrcMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        match self {
            IrcMessage::Numeric(_, msg)
            | IrcMessage::CAP(msg)
            | IrcMessage::JOIN(msg)
            | IrcMessage::MODE(msg)
            | IrcMessage::NOTICE(msg)
            | IrcMessage::PING(msg)
            | IrcMessage::PONG(msg)
            | IrcMessage::PRIVMSG(msg)
            | IrcMessage::VERSION(msg)
            | IrcMessage::QUIT(msg) => msg,
        }
    }
}
