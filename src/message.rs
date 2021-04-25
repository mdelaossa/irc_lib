use std::str::FromStr;

use irc_rust::Message;

#[derive(Debug, Clone)]
pub struct IrcMessage {
    raw: Message,
    pub command: IrcMessageType
}

#[derive(Debug, PartialEq, Clone)]
pub enum IrcMessageType {
    Numeric(u32),
    CAP,
    JOIN,
    MODE,
    NOTICE,
    PING,
    PONG,
    PRIVMSG,
    VERSION,
}

impl IrcMessage {
    pub fn from(str: &str) -> Self {
        let msg = Message::from(str);

        Self {
            command: IrcMessageType::from_str(msg.command()).unwrap(),
            raw: msg
        }
    }

    pub fn to_string(&self) -> String {
        self.raw.to_string()
    }

    pub fn params(&self) -> Option<irc_rust::Params> {
        self.raw.params()
    }

    pub fn prefix(&self) -> Result<Option<irc_rust::Prefix>, irc_rust::InvalidIrcFormatError> {
        self.raw.prefix()
    }
}

impl std::str::FromStr for IrcMessageType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CAP" => Ok(IrcMessageType::CAP),
            "JOIN" => Ok(IrcMessageType::JOIN),
            "MODE" => Ok(IrcMessageType::MODE),
            "NOTICE" => Ok(IrcMessageType::NOTICE),
            "PING" => Ok(IrcMessageType::PING),
            "PONG" => Ok(IrcMessageType::PONG),
            "PRIVMSG" => Ok(IrcMessageType::PRIVMSG),
            "VERSION" => Ok(IrcMessageType::VERSION),
            _ => {
                if let Ok(num) = s.parse::<u32>() {
                    Ok(IrcMessageType::Numeric(num))
                } else {
                    Err(format!("Failed to convert {} into IrcMessageType", s))
                }
            }
        }
    }
}