use std::collections::HashMap;

use crate::connection::Connection;
use crate::server::Channel;
use crate::{IrcPlugin, Server};

#[derive(Debug)]
pub struct Config {
    pub(crate) server: String,
    pub(crate) nick: String,
    pub(crate) user: String,
    pub(crate) channels: HashMap<String, Channel>,
    pub(crate) plugins: Vec<Box<dyn IrcPlugin>>,
}

impl Config {
    pub fn new(server: &str) -> Self {
        Config {
            server: server.to_owned(),
            nick: "User".to_owned(),
            user: "rusty".to_owned(),
            channels: HashMap::new(),
            plugins: Vec::new(),
        }
    }

    pub fn nick(mut self, nick: &str) -> Self {
        self.nick = nick.to_owned();

        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.user = user.to_owned();

        self
    }

    pub fn channel(mut self, channel: &str) -> Self {
        let channel: Channel = channel.parse().unwrap();
        self.channels.insert(channel.name.clone(), channel);

        self
    }

    pub fn register_plugin(mut self, plugin: impl IrcPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));

        self
    }

    pub fn build(self) -> Server {
        Server::new(self, Box::new(Connection::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new("irc.example.com")
            .nick("rusty")
            .user("rusty")
            .channel("#channel")
            .channel("#other");

        assert_eq!(config.server, "irc.example.com");
        assert_eq!(config.nick, "rusty");
        assert_eq!(config.user, "rusty");
        assert_eq!(config.channels.len(), 2);

        config.build();
    }
}
