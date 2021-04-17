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
impl irc_lib::IrcPlugin for BasicPlugin {
    fn message(&self, message: &irc_lib::IrcMessage) {
        println!("Plugin received message {:?}", message)
    }
}

fn main() {
    let mut config = irc_lib::Client::new("irc.subluminal.net:6667");
    config.nick("rusty_test")
        .channel("#test_123")
        .register_plugin(BasicPlugin);
        
    config.build().run()
 }
