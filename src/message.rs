use std::{ops::Deref, str::FromStr};

#[derive(Debug, Clone)]
pub struct Message {
    raw: irc_rust::Message,
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
}

impl Message {
    fn from(str: &str) -> Self {
        let msg = irc_rust::Message::from(str);

        Self { raw: msg }
    }
}

impl Deref for Message {
    type Target = irc_rust::Message;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl IrcMessage {
    // so users of our crate don't have to `use std::FromStr`
    pub fn from(s: &str) -> Result<Self, String> {
        Self::from_str(s)
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
            | IrcMessage::VERSION(msg) => msg,
        }
    }
}

impl FromStr for IrcMessage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let message = Message::from(s);
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
            _ => {
                if let Ok(num) = command.parse::<u32>() {
                    Ok(IrcMessage::Numeric(num, message))
                } else {
                    Err(format!("Failed to convert {} into IrcMessage", s))
                }
            }
        }
    }
}
