mod connection;
mod irc_plugin;
pub mod message;
mod server;

pub use irc_plugin::IrcPlugin;
pub use message::IrcMessage;
pub use server::Server;
pub use Config as Client;

#[derive(Debug)]
pub struct Config {
    server: String,
    nick: String,
    channels: Vec<String>,
    plugins: Vec<Box<dyn IrcPlugin>>
}

impl Config {
    pub fn new(server: &str) -> Self {
        Config {
            server: server.to_owned(),
            nick: "User".to_owned(),
            channels: Vec::new(),
            plugins: Vec::new()
        }
    }

    pub fn nick(&mut self, nick: &str) -> &mut Self {
        self.nick = nick.to_owned();
        
        self
    }

    pub fn channel(&mut self, channel: &str) -> &mut Self {
        self.channels.push(channel.to_owned());

        self
    }

    pub fn register_plugin(&mut self, plugin: impl IrcPlugin + 'static) -> &mut Self {
        self.plugins.push(Box::new(plugin));

        self
    }

    pub fn build(self) -> Server {
        Server::new(self)
    }
}
