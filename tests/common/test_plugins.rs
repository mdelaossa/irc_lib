use irc_lib::{IrcMessage, IrcPlugin, message};

#[derive(Debug)]
pub struct EchoPlugin;
impl IrcPlugin for EchoPlugin {
    fn message(&self, server: &irc_lib::Server, message: &IrcMessage) {
        if let IrcMessage {
            command: message::Command::PrivMsg,
            ..
        } = message
        {
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
    }
}
