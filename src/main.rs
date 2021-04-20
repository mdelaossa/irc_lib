use irc_lib::{IrcClient, IrcPlugin};
use irc_rust::Message;

// Expected API:

// impl IrcPLugin for STRUCT {
    // fn message(IrcClient, IrcMessage) {
        // IrcClient.send_message(some IrcMessage);
        // IrcCllient.{channels, users}
        // IrcChannel.users
    // }
// }
// 
// irc_lib::Client::new(CONFIG).run(); // This loops/runs everything

#[derive(Debug)]
struct BasicPlugin;
impl IrcPlugin for BasicPlugin {
    fn message(&self, server: &irc_lib::Server, message: &Message) {
        // Just an echo for now
        match message.command() {
            "PRIVMSG" => server.send_message(format!("PRIVMSG {}", message.params().unwrap()).as_str()),
            _ => ()
        }
    }
}

fn main() {
    let irc_client = IrcClient::new("192.168.33.10:6697")
        .nick("rusty_test")
        .channel("#test_123")
        .register_plugin(BasicPlugin)
        .build();

    // let (reader, sender) = irc_client.channels(); // thread channels

    // let ui = Ui::new();
    // loop {
    //     match ui::event {
    //         // send some stuff to irc

    //     }

    //     match reader.events() {
    //         // send some stuff to ui
    //     }
    // }
 }
