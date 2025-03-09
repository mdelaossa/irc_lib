use crate::{Config, server::Channel};

pub struct Negotiator {
    channels: std::collections::hash_map::IntoIter<String, Channel>,
    done: bool,
    messages: std::vec::IntoIter<String>,
}

impl Negotiator {
    pub fn new(config: &Config) -> Self {
        Negotiator {
            channels: config.channels.clone().into_iter(),
            done: false,
            messages: vec![
                "CAP LS 302".to_string(),
                format!("USER {} 0 * None", config.user),
                format!("NICK {}", config.nick),
                "CAP END".to_string(),
            ]
            .into_iter(),
        }
    }
}

impl Iterator for Negotiator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if let Some(n) = self.messages.next() {
            return Some(n);
        }

        if let Some((_, n)) = self.channels.next() {
            return Some(format!("JOIN {}", n));
        }

        self.done = true;
        None
    }
}
