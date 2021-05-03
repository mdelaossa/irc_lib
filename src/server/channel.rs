use std::{collections::HashMap, fmt::Display, str::FromStr};

use super::user::UserEntry;

#[derive(Clone, Debug, Default)]
pub struct Channel {
    pub name: String,
    pub users: HashMap<String, UserEntry>
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
        let name = if s.starts_with('#') {
            &s[1..]
        } else {
            &s[..]
        };

        Ok(Channel {
            name: name.to_owned(),
            ..Default::default()
        })
    }
}
