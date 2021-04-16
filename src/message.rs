pub struct IrcMessage<'a> {
    pub size: usize,
    pub text: &'a super::RawIrcMessage
}

impl IrcMessage<'_> {
    pub fn to_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.text[..self.size])
    }
}