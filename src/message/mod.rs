mod irc_message;
mod error;

pub use irc_message::{IrcMessage, Command as IrcCommand, Param as IrcMessageParam, Prefix as IrcMessagePrefix};
pub use error::Error;