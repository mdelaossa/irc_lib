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
    IrcClient::new("irc.subluminal.net:6667")
        .nick("rusty_test")
        .channel("#test_123")
        .register_plugin(BasicPlugin)
        .build()
        .run()
 }
