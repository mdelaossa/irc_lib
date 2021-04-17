use crate::message::IrcMessage;

use std::fmt::Debug;

pub trait IrcPlugin: Debug + Sync + Send {
    fn message(&self, message: &IrcMessage);
}