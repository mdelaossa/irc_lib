use std::{
    sync::{
        Arc, Condvar, Mutex,
        mpsc::{Receiver, Sender},
    },
    thread::JoinHandle,
};

use super::error::{Error, Result};
use crate::message::{Command, IrcMessage, Param};

#[derive(Debug)]
pub struct Client {
    pub(in crate::server) thread: Option<JoinHandle<()>>,
    pub(in crate::server) snd_channel: Option<Sender<IrcMessage>>,
    pub(in crate::server) rcv_channel: Option<Receiver<IrcMessage>>,
    pub(in crate::server) ready: Arc<(Mutex<bool>, Condvar)>,
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
        self.wait_ready();

        (
            self.snd_channel.as_ref().unwrap(),
            self.rcv_channel.as_ref().unwrap(),
        )
    }

    // Blocks until the connection is considered ready
    fn wait_ready(&self) {
        let (lock, cvar) = &*self.ready;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }

    pub fn shutdown(self) -> Result<()> {
        // Time to close our connection!
        if let Some(send) = &self.snd_channel {
            if let Ok(msg) = IrcMessage::builder()
                .command(Command::Quit)
                .param(Param::Message("Client shutting down".to_string()))
                .build()
            {
                send.send(msg).map_err(|_| Error::Send)?
            }
        }

        drop(self);
        Ok(())
    }
}
