use irc_lib::{IrcClient, IrcMessage, IrcPlugin, message};

#[derive(Debug)]
struct BasicPlugin;
impl IrcPlugin for BasicPlugin {
    fn message(&self, server: &irc_lib::Server, message: &IrcMessage) {
        // Just an echo for now
        println!("Plugin received message: {:?}", message);
        match message {
            IrcMessage {
                command: message::Command::PrivMsg,
                ..
            } => {
                if let (Some(content), Some(message::Prefix::User { nick: source, .. })) =
                    (message.get_message(), &message.prefix)
                {
                    let reply = format!("{}: {}", source, content);
                    let channel = message.get_channel().unwrap();
                    let msg = IrcMessage::builder()
                        .command(message::Command::PrivMsg)
                        .param(message::Param::Channel(channel.to_string()))
                        .param(message::Param::Message(reply))
                        .build()
                        .unwrap();
                    if let Err(e) = server.send_message(msg) {
                        println!("Error sending message: {:?}", e)
                    }
                }
            }
            _ => (),
        }
    }
}

fn main() {
    let irc_client = IrcClient::new("irc.subluminal.net:6667")
        .nick("rusty_test")
        .user("rustacean")
        .channel("#test_123")
        .register_plugin(BasicPlugin)
        .build()
        .run();

    let (sender, reader) = irc_client.channels(); // thread channels

    for message in reader.iter() {
        println!("Main thread received message: {:?}", message);

        // Echo!
        if let IrcMessage {
            command: message::Command::PrivMsg,
            params,
            ..
        } = message
        {
            let mut msg = IrcMessage::builder().command(message::Command::PrivMsg);
            for param in params {
                msg = msg.param(param.to_owned());
            }
            let msg = msg.build().unwrap();
            sender
                .send(msg)
                .expect("MAIN THREAD COULDN'T SEND IRC MESSAGE");
        }
    }
}
