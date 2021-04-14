use irc_lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut connection = irc_lib::Client::new("irc.subluminal.net:6667");
    connection.connect()?;
    let mut buff = [0; 512];

    let mut negotiator = irc_lib::connection::negotiator::Negotiator::new();

    loop {
        match connection.read(&mut buff) {
        Ok(size) => {
            if size == 0 {
                continue;
            }
            println!("{:?}", std::str::from_utf8(&buff[..size])?);

            if std::str::from_utf8(&buff[0..size])?.contains("PING") {
                connection.send_message(b"PONG")?;
            } else if std::str::from_utf8(&buff[0..size])?.contains("VERSION") {
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
