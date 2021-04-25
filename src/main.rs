use irc_lib::{IrcClient, IrcMessage, IrcMessageType, IrcPlugin};

// Expected API:

// impl IrcPLugin for STRUCT {
    // fn message(IrcClient, IrcMessage) {
        // IrcClient.send_message(some IrcMessage);
        // IrcCllient.{channels, users}
        // IrcChannel.users
        // IrcChannel.send_message(some IrcMessage);
    // }
// }
// 
// irc_lib::Client::new(CONFIG).run(); // This loops/runs everything

#[derive(Debug)]
struct BasicPlugin;
impl IrcPlugin for BasicPlugin {
    fn message(&self, server: &irc_lib::Server, message: &IrcMessage) {
        // Just an echo for now
        match message.command {
            IrcMessageType::PRIVMSG => server.send_message(format!("PRIVMSG {}", message.params().unwrap()).as_str()),
            _ => ()
        }
    }
}

fn main() {
    let irc_client = IrcClient::new("irc.subluminal.net:6667")
        .nick("rusty_test")
        .channel("#test_123")
        .register_plugin(BasicPlugin)
        .build()
        .run();
    
    let (sender, reader) = irc_client.channels(); // thread channels

    loop {
        for message in reader.try_iter() {
            println!("Main thread received message: {:?}", message);

            // Echo!
            if message.command == IrcMessageType::PRIVMSG {
                sender.send(IrcMessage::from(format!("PRIVMSG {}", message.params().unwrap()).as_str())).expect("MAIN THREAD COULDN'T SEND IRC MESSAGE");
            }
        }
    }
 }
