use std::{sync::mpsc::{Receiver, Sender}, thread::JoinHandle};

use crate::message::{IrcMessage, Command, Param};
use super::error::{ Result, Error };


#[derive(Debug)]
pub struct Client {
    pub(in crate::server) thread: Option<JoinHandle<()>>,
    pub(in crate::server) snd_channel: Option<Sender<IrcMessage>>,
    pub(in crate::server) rcv_channel: Option<Receiver<IrcMessage>>,
}

impl Drop for Client {
    fn drop(&mut self) {
        // If we're here, no one is gonna be using our channels, so let's clean up
        drop(self.snd_channel.take());
        drop(self.rcv_channel.take());

        // Join the thread so the server keeps running
        if let Some(thread) = self.thread.take() {
            thread
                .join()
                .expect("Critical error with IRC Client. Aborting");
        }
    }
}

impl Client {
    pub fn channels(&self) -> (&Sender<IrcMessage>, &Receiver<IrcMessage>) {
        (
            self.snd_channel.as_ref().unwrap(),
            self.rcv_channel.as_ref().unwrap(),
        )
    }

    pub fn shutdown(self) -> Result<()> {
        // Time to close our connection!
        if let Some(send) = &self.snd_channel {
            if let Ok(msg) = IrcMessage::builder()
            .command(Command::Quit)
            .param(Param::Message("Client shutting down".to_string()))
            .build() {
                send.send(msg).map_err(|_| Error::SendError)?
            }
        }

        drop(self);
        Ok(())
    }
}