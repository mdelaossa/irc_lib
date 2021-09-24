use regex::Regex;

#[derive(Clone, Debug)]
pub struct User {
    pub nick: String,
    pub r#type: UserType,
}

#[derive(Clone, Debug)]
pub enum UserType {
    Regular,
    Op,
    HalfOp,
}

impl User {
    pub(crate) fn new(nick: &str) -> Self {
        let (perm, nick) = Self::parse_nick(nick);

        let perm = if let Some(perm) = perm {
            match perm.as_str() {
                "@" => UserType::Op,
                "%" => UserType::HalfOp,
                _ => unreachable!(),
            }
        } else {
            UserType::Regular
        };
        
        User {
            nick: nick.to_string(),
            r#type: perm
        }
    }

    fn parse_nick(str: &str) -> (Option<regex::Match>, &str) {
        let re = Regex::new(r"([@%])?(\w+)").unwrap();
        let caps = re.captures(str).unwrap();

        (caps.get(1), caps.get(2).unwrap().as_str())
    }
}
