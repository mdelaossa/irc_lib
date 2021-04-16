fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = irc_lib::Client::new("irc.subluminal.net:6667");
    connection.connect()?;

    let mut negotiator = irc_lib::connection::negotiator::Negotiator::new();

    loop {
        match connection.read() {
        Ok(message) => {
            println!("{:?}", std::str::from_utf8(&message.text[..message.size])?);

            if message.to_utf8()?.contains("PING") {
                connection.send_message(b"PONG")?;
            } else if message.to_utf8()?.contains("VERSION") {
                connection.send_message(b"VERSION 123")?;
            } else if let Some(message) = negotiator.next() {
                connection.send_message(message.as_bytes())?;
            }else {
                connection.send_message(b"hi")?;
            }
        },
        Err(err) => return Err(Box::new(err))
        }
    }
 }
