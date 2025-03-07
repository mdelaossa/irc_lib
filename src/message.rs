use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct IrcMessage {
    pub prefix: Option<Prefix>,
    pub command: Command,
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Prefix {
    Server(String),
    User {
        nick: String,
        user: Option<String>,
        host: Option<String>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Command {
    Join,
    Part,
    PrivMsg,
    Notice,
    Nick,
    User,
    Quit,
    Ping,
    Pong,
    Mode,
    Topic,
    Invite,
    Kick,
    Motd,
    Lusers,
    Version,
    Stats,
    Links,
    Time,
    Connect,
    Trace,
    Admin,
    Info,
    Servlist,
    Squery,
    Whois,
    Whowas,
    Kill,
    Error,
    Away,
    Rehash,
    Restart,
    Summon,
    Users,
    Wallops,
    Userhost,
    Ison,
    Numeric(u16),
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Param {
    Channel(String),
    Message(String),
    Nick(String),
    User(String, String, String, String),
    Unknown(String),
}

impl IrcMessage {
    pub fn new(prefix: Option<Prefix>, command: Command, params: Vec<Param>) -> Self {
        IrcMessage {
            prefix,
            command,
            params,
        }
    }

    pub fn from_str(input: &str) -> Result<Self, &'static str> {
        let mut parts = input.split_whitespace();
        let prefix = if input.starts_with(':') {
            parts.next().map(|s| s[1..].to_string()).and_then(|s| {
                if s.contains('!') && s.contains('@') {
                    let mut user_parts = s.split('!');
                    let nick = user_parts.next()?.to_string();
                    let mut host_parts = user_parts.next()?.split('@');
                    let user = host_parts.next()?.to_string();
                    let host = host_parts.next()?.to_string();
                    Some(Prefix::User {
                        nick,
                        user: Some(user),
                        host: Some(host),
                    })
                } else if s.contains('.') {
                    Some(Prefix::Server(s))
                } else {
                    Some(Prefix::User {
                        nick: s,
                        user: None,
                        host: None,
                    })
                }
            })
        } else {
            None
        };

        let command_str = parts.next().ok_or("Missing command")?.to_string();
        let command = match command_str.as_str() {
            "JOIN" => Command::Join,
            "PART" => Command::Part,
            "PRIVMSG" => Command::PrivMsg,
            "NOTICE" => Command::Notice,
            "NICK" => Command::Nick,
            "USER" => Command::User,
            "QUIT" => Command::Quit,
            "PING" => Command::Ping,
            "PONG" => Command::Pong,
            "MODE" => Command::Mode,
            "TOPIC" => Command::Topic,
            "INVITE" => Command::Invite,
            "KICK" => Command::Kick,
            "MOTD" => Command::Motd,
            "LUSERS" => Command::Lusers,
            "VERSION" => Command::Version,
            "STATS" => Command::Stats,
            "LINKS" => Command::Links,
            "TIME" => Command::Time,
            "CONNECT" => Command::Connect,
            "TRACE" => Command::Trace,
            "ADMIN" => Command::Admin,
            "INFO" => Command::Info,
            "SERVLIST" => Command::Servlist,
            "SQUERY" => Command::Squery,
            "WHOIS" => Command::Whois,
            "WHOWAS" => Command::Whowas,
            "KILL" => Command::Kill,
            "ERROR" => Command::Error,
            "AWAY" => Command::Away,
            "REHASH" => Command::Rehash,
            "RESTART" => Command::Restart,
            "SUMMON" => Command::Summon,
            "USERS" => Command::Users,
            "WALLOPS" => Command::Wallops,
            "USERHOST" => Command::Userhost,
            "ISON" => Command::Ison,
            _ if command_str.chars().all(|c| c.is_digit(10)) => {
                Command::Numeric(command_str.parse().unwrap_or(0))
            }
            _ => Command::Unknown(command_str),
        };

        let params_str = parts.collect::<Vec<_>>().join(" ");
        let params = IrcMessage::parse_params(&command, &params_str);

        Ok(IrcMessage {
            prefix,
            command,
            params,
        })
    }

    fn parse_params(command: &Command, params_str: &str) -> Vec<Param> {
        let mut params = Vec::new();
        let mut parts = params_str.split_whitespace();

        match command {
            Command::Join | Command::Part => {
                if let Some(channel) = parts.next() {
                    params.push(Param::Channel(channel.to_string()));
                }
            }
            Command::PrivMsg | Command::Notice => {
                if let Some(channel) = parts.next() {
                    params.push(Param::Channel(channel.to_string()));
                }
                if let Some(message) = parts.collect::<Vec<&str>>().join(" ").strip_prefix(':') {
                    params.push(Param::Message(message.to_string()));
                }
            }
            Command::Nick => {
                if let Some(nick) = parts.next() {
                    params.push(Param::Nick(nick.to_string()));
                }
            }
            Command::User => {
                if let (Some(username), Some(hostname), Some(servername), Some(realname)) = (
                    parts.next(),
                    parts.next(),
                    parts.next(),
                    parts.collect::<Vec<&str>>().join(" ").strip_prefix(':'),
                ) {
                    params.push(Param::User(
                        username.to_string(),
                        hostname.to_string(),
                        servername.to_string(),
                        realname.to_string(),
                    ));
                }
            }
            Command::Quit => {
                if let Some(message) = parts.collect::<Vec<&str>>().join(" ").strip_prefix(':') {
                    params.push(Param::Message(message.to_string()));
                }
            }
            Command::Ping | Command::Pong => {
                if let Some(message) = parts.next() {
                    params.push(Param::Message(message[1..].to_string()));
                }
            }
            Command::Unknown(_) | Command::Numeric(_) => {
                for part in parts {
                    params.push(Param::Unknown(part.to_string()));
                }
            }
            _ => {}
        }

        params
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref prefix) = self.prefix {
            result.push(':');
            result.push_str(&prefix.to_string());
            result.push(' ');
        }
        result.push_str(&self.command.to_string());
        for param in &self.params {
            result.push(' ');
            result.push_str(&param.to_string());
        }
        result
    }

    pub fn builder() -> IrcMessageBuilder {
        IrcMessageBuilder::new()
    }

    pub fn get_message(&self) -> Option<&String> {
        self.params.iter().find_map(|param| {
            if let Param::Message(ref msg) = param {
                Some(msg)
            } else {
                None
            }
        })
    }

    pub fn get_channel(&self) -> Option<&String> {
        self.params.iter().find_map(|param| {
            if let Param::Channel(ref ch) = param {
                Some(ch)
            } else {
                None
            }
        })
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Prefix::Server(server) => write!(f, "{}", server),
            Prefix::User { nick, user, host } => {
                if let (Some(user), Some(host)) = (user, host) {
                    write!(f, "{}!{}@{}", nick, user, host)
                } else {
                    write!(f, "{}", nick)
                }
            }
        }
    }
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let command_str = match self {
            Command::Join => "JOIN".to_string(),
            Command::Part => "PART".to_string(),
            Command::PrivMsg => "PRIVMSG".to_string(),
            Command::Notice => "NOTICE".to_string(),
            Command::Nick => "NICK".to_string(),
            Command::User => "USER".to_string(),
            Command::Quit => "QUIT".to_string(),
            Command::Ping => "PING".to_string(),
            Command::Pong => "PONG".to_string(),
            Command::Mode => "MODE".to_string(),
            Command::Topic => "TOPIC".to_string(),
            Command::Invite => "INVITE".to_string(),
            Command::Kick => "KICK".to_string(),
            Command::Motd => "MOTD".to_string(),
            Command::Lusers => "LUSERS".to_string(),
            Command::Version => "VERSION".to_string(),
            Command::Stats => "STATS".to_string(),
            Command::Links => "LINKS".to_string(),
            Command::Time => "TIME".to_string(),
            Command::Connect => "CONNECT".to_string(),
            Command::Trace => "TRACE".to_string(),
            Command::Admin => "ADMIN".to_string(),
            Command::Info => "INFO".to_string(),
            Command::Servlist => "SERVLIST".to_string(),
            Command::Squery => "SQUERY".to_string(),
            Command::Whois => "WHOIS".to_string(),
            Command::Whowas => "WHOWAS".to_string(),
            Command::Kill => "KILL".to_string(),
            Command::Error => "ERROR".to_string(),
            Command::Away => "AWAY".to_string(),
            Command::Rehash => "REHASH".to_string(),
            Command::Restart => "RESTART".to_string(),
            Command::Summon => "SUMMON".to_string(),
            Command::Users => "USERS".to_string(),
            Command::Wallops => "WALLOPS".to_string(),
            Command::Userhost => "USERHOST".to_string(),
            Command::Ison => "ISON".to_string(),
            Command::Numeric(num) => format!("{:03}", num),
            Command::Unknown(cmd) => cmd.clone(),
        };
        write!(f, "{}", command_str)
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let param_str = match self {
            Param::Channel(ch) => ch.clone(),
            Param::Message(msg) => msg.clone(),
            Param::Nick(nick) => nick.clone(),
            Param::User(username, hostname, servername, realname) => {
                format!("{} {} {} :{}", username, hostname, servername, realname)
            }
            Param::Unknown(param) => param.clone(),
        };
        write!(f, "{}", param_str)
    }
}

pub struct IrcMessageBuilder {
    prefix: Option<Prefix>,
    command: Option<Command>,
    params: Vec<Param>,
}

impl IrcMessageBuilder {
    pub fn new() -> Self {
        IrcMessageBuilder {
            prefix: None,
            command: None,
            params: Vec::new(),
        }
    }

    pub fn prefix(mut self, prefix: &str) -> Self {
        let prefix_struct = if prefix.contains('!') && prefix.contains('@') {
            let mut user_parts = prefix.split('!');
            let nick = user_parts.next().unwrap().to_string();
            let mut host_parts = user_parts.next().unwrap().split('@');
            let user = host_parts.next().unwrap().to_string();
            let host = host_parts.next().unwrap().to_string();
            Prefix::User {
                nick,
                user: Some(user),
                host: Some(host),
            }
        } else if prefix.contains('.') {
            Prefix::Server(prefix.to_string())
        } else {
            Prefix::User {
                nick: prefix.to_string(),
                user: None,
                host: None,
            }
        };
        self.prefix = Some(prefix_struct);
        self
    }

    pub fn command(mut self, command: Command) -> Self {
        self.command = Some(command);
        self
    }

    pub fn param(mut self, param: Param) -> Self {
        self.params.push(param);
        self
    }

    pub fn build(self) -> Result<IrcMessage, &'static str> {
        if let Some(command) = self.command {
            Ok(IrcMessage {
                prefix: self.prefix,
                command,
                params: self.params,
            })
        } else {
            Err("Command is required")
        }
    }
}

impl fmt::Display for IrcMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = ":prefix JOIN #channel";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }));
        assert_eq!(msg.command, Command::Join);
        assert_eq!(msg.params, vec![Param::Channel("#channel".to_string())]);
    }

    #[test]
    fn test_to_string() {
        let msg = IrcMessage::new(
            Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }),
            Command::Join,
            vec![Param::Channel("#channel".to_string())],
        );
        assert_eq!(msg.to_string(), ":prefix JOIN #channel");
    }

    #[test]
    fn test_builder() {
        let msg = IrcMessage::builder()
            .prefix("prefix")
            .command(Command::Join)
            .param(Param::Channel("#channel".to_string()))
            .build()
            .unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }));
        assert_eq!(msg.command, Command::Join);
        assert_eq!(msg.params, vec![Param::Channel("#channel".to_string())]);
        assert_eq!(msg.to_string(), ":prefix JOIN #channel");
    }

    #[test]
    fn test_display() {
        let msg = IrcMessage::new(
            Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }),
            Command::Join,
            vec![Param::Channel("#channel".to_string())],
        );
        assert_eq!(format!("{}", msg), ":prefix JOIN #channel");
    }

    #[test]
    fn test_privmsg_with_message() {
        let input = ":nick!user@some.server PRIVMSG #channel :Hello, world!";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "nick".to_string(), user: Some("user".to_string()), host: Some("some.server".to_string()) }));
        assert_eq!(msg.command, Command::PrivMsg);
        assert_eq!(
            msg.params,
            vec![
                Param::Channel("#channel".to_string()),
                Param::Message("Hello, world!".to_string())
            ]
        );
    }

    #[test]
    fn test_notice_with_message() {
        let input = ":prefix NOTICE #channel :Hello, world!";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }));
        assert_eq!(msg.command, Command::Notice);
        assert_eq!(
            msg.params,
            vec![
                Param::Channel("#channel".to_string()),
                Param::Message("Hello, world!".to_string())
            ]
        );
    }

    #[test]
    fn test_user_command() {
        let input = "USER guest tolmoon tolsun :Ronnie Reagan";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, None);
        assert_eq!(msg.command, Command::User);
        assert_eq!(
            msg.params,
            vec![Param::User(
                "guest".to_string(),
                "tolmoon".to_string(),
                "tolsun".to_string(),
                "Ronnie Reagan".to_string()
            )]
        );
    }

    #[test]
    fn test_quit_command() {
        let input = "QUIT :Gone to have lunch";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, None);
        assert_eq!(msg.command, Command::Quit);
        assert_eq!(
            msg.params,
            vec![Param::Message("Gone to have lunch".to_string())]
        );
    }

    #[test]
    fn test_ping_command() {
        let input = "PING :server1";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, None);
        assert_eq!(msg.command, Command::Ping);
        assert_eq!(msg.params, vec![Param::Message("server1".to_string())]);
    }

    #[test]
    fn test_pong_command() {
        let input = "PONG :server1";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, None);
        assert_eq!(msg.command, Command::Pong);
        assert_eq!(msg.params, vec![Param::Message("server1".to_string())]);
    }

    #[test]
    fn test_numeric_command() {
        let input = ":prefix 001 Welcome to the IRC network";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }));
        assert_eq!(msg.command, Command::Numeric(1));
        assert_eq!(
            msg.params,
            vec![
                Param::Unknown("Welcome".to_string()),
                Param::Unknown("to".to_string()),
                Param::Unknown("the".to_string()),
                Param::Unknown("IRC".to_string()),
                Param::Unknown("network".to_string())
            ]
        );
    }

    #[test]
    fn test_unknown_command() {
        let input = ":prefix UNKNOWNCMD some parameters";
        let msg = IrcMessage::from_str(input).unwrap();
        assert_eq!(msg.prefix, Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }));
        assert_eq!(msg.command, Command::Unknown("UNKNOWNCMD".to_string()));
        assert_eq!(
            msg.params,
            vec![
                Param::Unknown("some".to_string()),
                Param::Unknown("parameters".to_string())
            ]
        );
    }

    #[test]
    fn test_get_message() {
        let msg = IrcMessage::new(
            Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }),
            Command::PrivMsg,
            vec![
                Param::Channel("#channel".to_string()),
                Param::Message("Hello, world!".to_string()),
            ],
        );
        assert_eq!(msg.get_message(), Some(&"Hello, world!".to_string()));
    }

    #[test]
    fn test_get_channel() {
        let msg = IrcMessage::new(
            Some(Prefix::User { nick: "prefix".to_string(), user: None, host: None }),
            Command::PrivMsg,
            vec![
                Param::Channel("#channel".to_string()),
                Param::Message("Hello, world!".to_string()),
            ],
        );
        assert_eq!(msg.get_channel(), Some(&"#channel".to_string()));
    }
}
