use crate::{Config, server::channel::Channel};

pub struct Negotiator {
    channels: std::collections::hash_map::IntoIter<String, Channel>,
    done: bool,
    messages: std::slice::Iter<'static, &'static str>
}

impl Negotiator {
    pub fn new(config: &Config) -> Self {
        Negotiator {
            channels: config.channels.clone().into_iter(),
            done: false,
            messages: [
                "CAP LS 302",
                "USER rusty 0 * None",
                "NICK rusty_nick",
                "CAP END"
            ].iter()        
        }
    }
}

impl Iterator for Negotiator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.done { return None }
        
        match self.messages.next() {
            Some(n) => Some(n.to_string()),
            None => {
                match self.channels.next() {
                    Some((_, n)) => Some(format!("JOIN {}", n)),
                    None => {
                        self.done = true;
                        None
                    }
                }
            }
        }
    }
}