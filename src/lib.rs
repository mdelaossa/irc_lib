mod config;
mod connection;
mod irc_plugin;
pub mod message;
mod server;

pub use config::Config as IrcClient;
pub use irc_plugin::IrcPlugin;
pub use message::IrcMessage;
pub use server::*;

pub(crate) use config::Config;

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
