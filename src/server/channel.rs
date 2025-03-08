use std::{collections::HashMap, fmt::Display, str::FromStr};

use super::user::User;

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub name: String,
    pub users: HashMap<String, User>,
}

impl Channel {
    pub(crate) fn new(name: &str) -> Self {
        Self::from_str(name).unwrap()
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.name)
    }
}

impl FromStr for Channel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let name = if let Some(str) = s.strip_prefix('#') {
            str
        } else {
            s
        };

        Ok(Channel {
            name: name.to_owned(),
            ..Default::default()
        })
    }
}
