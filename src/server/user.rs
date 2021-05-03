use std::ops::Deref;

use regex::Regex;

#[derive(Clone, Debug)]
pub struct User {
    pub nick: String,
}

#[derive(Clone, Debug)]
pub enum UserEntry {
    Regular(User),
    Op(User),
    HalfOp(User)
}

impl User {
    pub(crate) fn new(nick: &str) -> UserEntry {

        let (perm, nick) = Self::parse_nick(nick);
        let user = User { nick: nick.to_string() };

        if let Some(perm) = perm {
            match perm.as_str() {
                "@" => UserEntry::Op(user),
                "%" => UserEntry::HalfOp(user),
                _ => unreachable!(),
            }
        } else {
            UserEntry::Regular(user)
        }
    }

    fn parse_nick(str: &str) -> (Option<regex::Match>, &str) {
        let re = Regex::new(r"([@%])?(\w+)").unwrap();
        let caps = re.captures(str).unwrap();

        (
            caps.get(1),
            caps.get(2).unwrap().as_str()
        )
    }
}

impl Deref for UserEntry {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        match self {
            UserEntry::Regular(user)
            | UserEntry::Op(user) 
            | UserEntry::HalfOp(user) => user
        }
    }
}