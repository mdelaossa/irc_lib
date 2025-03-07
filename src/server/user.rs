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

        let perm = match perm {
            Some('@') => UserType::Op,
            Some('%') => UserType::HalfOp,
            _ => UserType::Regular,
        };
        
        User {
            nick: nick.to_string(),
            r#type: perm
        }
    }

    fn parse_nick(input: &str) -> (Option<char>, &str) {
        let mut chars = input.chars();
        let prefix = match chars.next() {
            Some(c) if c == '@' || c == '%' => Some(c),
            Some(_c) => None,
            None => return (None, ""),
        };
        (prefix, chars.as_str())
    }
}
