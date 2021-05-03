mod connection;
mod irc_plugin;
mod server;
mod message;

use std::{collections::HashMap, str::FromStr};

pub use message::IrcMessage;
pub use irc_plugin::IrcPlugin;
pub use server::Server;
pub use Config as IrcClient;

use server::channel::Channel;

#[derive(Debug)]
pub struct Config {
    server: String,
    nick: String,
    channels: HashMap<String, Channel>,
    plugins: Vec<Box<dyn IrcPlugin>>
}

impl Config {
    pub fn new(server: &str) -> Self {
        Config {
            server: server.to_owned(),
            nick: "User".to_owned(),
            channels: HashMap::new(),
            plugins: Vec::new()
        }
    }

    pub fn nick(mut self, nick: &str) -> Self {
        self.nick = nick.to_owned();
        
        self
    }

    pub fn channel(mut self, channel: &str) -> Self {
        let channel = Channel::from_str(channel).unwrap();
        self.channels.insert(channel.name.clone(), channel);

        self
    }

    pub fn register_plugin(mut self, plugin: impl IrcPlugin + 'static) -> Self {
        self.plugins.push(Box::new(plugin));

        self
    }

    pub fn build(self) -> Server {
        Server::new(self)
    }
}
