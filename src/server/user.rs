#[derive(Debug)]
pub struct User {
    pub nick: String,
    pub r#type: UserType
}

#[derive(Debug)]
pub enum UserType {
    Regular,
    Op,
    HalfOp
}