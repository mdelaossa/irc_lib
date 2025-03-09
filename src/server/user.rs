#[derive(Clone, Debug)]
pub struct User {
    pub nick: String,
    pub r#type: UserType,
}

#[derive(Clone, Debug, PartialEq)]
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
            r#type: perm,
        }
    }

    fn parse_nick(input: &str) -> (Option<char>, &str) {
        let mut chars = input.chars();
        let prefix = match chars.next() {
            Some(c) if c == '@' || c == '%' => Some(c),
            Some(_c) => return (None, input),
            None => return (None, ""),
        };
        (prefix, chars.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nick() {
        assert_eq!(User::parse_nick("nick"), (None, "nick"));
        assert_eq!(User::parse_nick("@nick"), (Some('@'), "nick"));
        assert_eq!(User::parse_nick("%nick"), (Some('%'), "nick"));
    }

    #[test]
    fn test_new() {
        let user = User::new("nick");
        assert_eq!(user.nick, "nick");
        assert_eq!(user.r#type, UserType::Regular);

        let user = User::new("@nick");
        assert_eq!(user.nick, "nick");
        assert_eq!(user.r#type, UserType::Op);

        let user = User::new("%nick");
        assert_eq!(user.nick, "nick");
        assert_eq!(user.r#type, UserType::HalfOp);
    }
}
