# IRC Lib - A simple IRC protocol library with plugin support

> [!CAUTION]
> Versions 0.1.x *do not follow SEMVER* and might have breaking changes. Make sure to pin your version! 0.2.x onwards will follow SEMVER.

## Key Features
- Plugin support
- Direct usage support
- Full IRC message building

## Examples

```rust,no_run
//! Base example
use irc_lib::{message, IrcClient, IrcMessage};

let irc_client = IrcClient::new("irc.server.net:6667")
        .nick("yourNickname")
        .user("username")
        .channel("#a_channel")
        .build()
        .run();

    let (sender, reader) = irc_client.channels(); // thread channels

    for message in reader.iter() {
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
```

```rust,no_run
//! Plugin example

use irc_lib::{message, Server, IrcClient, IrcMessage, IrcPlugin};

// First, implement your plugin
#[derive(Debug)]
struct BasicPlugin;
impl IrcPlugin for BasicPlugin {
    fn message(&self, server: &Server, message: &IrcMessage) {
        match message {
            IrcMessage {
                command: message::Command::PrivMsg,
                ..
            } => {
                // Echo prepending the user's nickname
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
            // You can match any other message type here
            _ => (),
        }
    }
}

// Next, configure the client

fn main() {
    let irc_client = IrcClient::new("irc.server.net:6667")
        .nick("yourNickname")
        .user("username")
        .channel("#a_channel")
        .register_plugin(BasicPlugin)
        .build()
        .run();
}
```