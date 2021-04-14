pub struct IRCMessage<'a> {
    pub size: usize,
    pub text: &'a super::RawIRCMessage
}

impl IRCMessage<'_> {
    pub fn to_utf8(&self) -> Result<&str, std::str::Utf8Error> {
        std::str::from_utf8(&self.text[..self.size])
    }
}