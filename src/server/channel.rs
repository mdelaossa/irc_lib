use super::user::User;

#[derive(Debug)]
pub struct Channel {
    pub users: Vec<User>
}