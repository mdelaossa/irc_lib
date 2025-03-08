use crate::{message::IrcMessage, Server};

use std::fmt::Debug;

pub trait IrcPlugin: Debug + Send {
    fn message(&self, server: &Server, message: &IrcMessage);
}
