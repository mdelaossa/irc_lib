mod common;

use irc_lib::{IrcClient, IrcMessage, message};
use std::time::Duration;

#[test]
fn send_message() -> common::TestResult {
    let mut errors = Vec::new();
    let mut harness = common::TestHarness::new();
    harness.start_ircd();
    let host = &harness.get_host();
    let channel = "#test";

    let irc_sender = harness.register_client(
        IrcClient::new(host)
            .nick("sender")
            .channel(channel)
            .build()
            .run(),
    );
    let irc_receiver = harness.register_client(
        IrcClient::new(host)
            .nick("receiver")
            .channel(channel)
            .build()
            .run(),
    );

    let (sender, _sender_receiver) = irc_sender.channels();
    let (_receiver_sender, receiver) = irc_receiver.channels();

    let msg = IrcMessage::builder()
        .command(message::Command::PrivMsg)
        .param(message::Param::Channel(String::from(channel)))
        .param(irc_lib::message::Param::Message(String::from(
            "Hello, world!",
        )))
        .build()
        .unwrap();

    common::clear_buffer(receiver);

    sender
        .send(msg)
        .expect("MAIN THREAD COULDN'T SEND IRC MESSAGE");

    // Verify that the message was actually sent
    if let Ok(received_msg) = receiver.recv_timeout(Duration::from_secs(2)) {
        let expected = "Hello, world!";
        let actual = received_msg.get_message().unwrap();
        if expected != actual {
            errors.push(format!("Expected: {}, but got {}", expected, actual))
        }
    } else {
        errors.push(String::from("Did not receive message"));
    }

    if !errors.is_empty() {
        return Err(errors.join("; ").into());
    }

    Ok(())
}

#[test]
fn plugin_replies_to_messages() -> common::TestResult {
    let mut errors = Vec::new();
    let mut harness = common::TestHarness::new();
    harness.start_ircd();
    let host = &harness.get_host();
    let channel = "#test";

    let irc_sender = harness.register_client(
        IrcClient::new(host)
            .nick("sender")
            .channel(channel)
            .build()
            .run(),
    );
    // replier
    harness.register_client(
        IrcClient::new(host)
            .nick("receiver")
            .channel(channel)
            .register_plugin(common::EchoPlugin)
            .build()
            .run(),
    );

    let (sender, sender_receiver) = irc_sender.channels();

    let msg = IrcMessage::builder()
        .command(message::Command::PrivMsg)
        .param(message::Param::Channel(String::from("#test")))
        .param(irc_lib::message::Param::Message(String::from(
            "Hello, world!",
        )))
        .build()
        .unwrap();

    common::clear_buffer(sender_receiver);
    sender
        .send(msg)
        .expect("MAIN THREAD COULDN'T SEND IRC MESSAGE");

    // Check for the echoed message
    if let Ok(received_msg) = sender_receiver.recv_timeout(Duration::from_secs(2)) {
        let expected = "sender: Hello, world!";
        let actual = received_msg.get_message().unwrap();
        if expected != actual {
            errors.push(format!("Expected: {}, but got {}", expected, actual))
        }
    } else {
        errors.push(String::from("Did not receive echoed message"));
    }

    if !errors.is_empty() {
        return Err(errors.join("; ").into());
    }

    Ok(())
}

#[test]
fn send_multiple_messages() -> common::TestResult {
    let mut errors = Vec::new();
    let mut harness = common::TestHarness::new();
    harness.start_ircd();
    let host = &harness.get_host();
    let channel = "#test";

    let irc_sender = harness.register_client(
        IrcClient::new(host)
            .nick("sender")
            .channel(channel)
            .build()
            .run(),
    );
    let irc_receiver = harness.register_client(
        IrcClient::new(host)
            .nick("receiver")
            .channel(channel)
            .build()
            .run(),
    );

    let (sender, _sender_receiver) = irc_sender.channels();
    let (_receiver_sender, receiver) = irc_receiver.channels();

    common::clear_buffer(receiver);

    for i in 0..5 {
        let msg = IrcMessage::builder()
            .command(message::Command::PrivMsg)
            .param(message::Param::Channel(String::from(channel)))
            .param(irc_lib::message::Param::Message(format!("Message {}", i)))
            .build()
            .unwrap();

        sender
            .send(msg)
            .expect("MAIN THREAD COULDN'T SEND IRC MESSAGE");
    }

    // Verify that the messages were received by the server
    for i in 0..5 {
        if let Ok(received_msg) = receiver.recv_timeout(Duration::from_secs(2)) {
            let expected = &format!("Message {}", i);
            let actual = received_msg.get_message().unwrap();
            if expected != actual {
                errors.push(format!("Expected: {}, but got {}", expected, actual))
            }
        } else {
            errors.push(String::from("Did not receive message"));
        }
    }

    if !errors.is_empty() {
        return Err(errors.join("; ").into());
    }

    Ok(())
}
