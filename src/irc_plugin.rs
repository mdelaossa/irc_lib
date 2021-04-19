use irc_rust::Message;

use crate::{Server};

use std::fmt::Debug;

pub trait IrcPlugin: Debug + Send {
    fn message(&self, server: &Server, message: &Message);
}