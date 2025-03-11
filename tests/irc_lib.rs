mod common;

use irc_lib::{IrcClient, IrcMessage, message};
use std::{collections::HashSet, time::Duration};

#[test]
fn send_message() {
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

    let error = format!("Could not send message: {:?}", msg);
    sender.send(msg).expect(&error);

    while let Ok(message) = receiver.recv_timeout(Duration::from_secs(1)) {
        if let IrcMessage {
            command: message::Command::PrivMsg,
            ..
        } = message
        {
            assert_eq!("Hello, world!", message.get_message().unwrap());
            return;
        }
    }

    panic!("Did not receive any messages");
}

#[test]
fn plugin_replies_to_messages() {
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
    let _replier = harness.register_client(
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

    let error = format!("Could not send message: {:?}", msg);
    sender.send(msg).expect(&error);

    while let Ok(message) = sender_receiver.recv_timeout(Duration::from_secs(1)) {
        if let IrcMessage {
            command: message::Command::PrivMsg,
            ..
        } = message
        {
            assert_eq!("sender: Hello, world!", message.get_message().unwrap());
            return;
        }
    }

    panic!("Did not receive any messages");
}

#[test]
fn send_multiple_messages() {
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

    for i in 0..5 {
        let msg = IrcMessage::builder()
            .command(message::Command::PrivMsg)
            .param(message::Param::Channel(String::from(channel)))
            .param(irc_lib::message::Param::Message(format!("Message {}", i)))
            .build()
            .unwrap();

        let error = format!("Could not send message: {:?}", msg);
        sender.send(msg).expect(&error);
    }

    let mut expected: HashSet<String> = (0..5).map(|i| format!("Message {}", i)).collect();
    while let Ok(message) = receiver.recv_timeout(Duration::from_secs(2)) {
        if let IrcMessage {
            command: message::Command::PrivMsg,
            ..
        } = message
        {
            assert!(
                expected.remove(message.get_message().unwrap()),
                "Unexpected message: {}",
                message
            );
            if expected.is_empty() {
                return;
            }
        }
    }
    assert!(
        expected.is_empty(),
        "Some expected messages were missing: {:?}",
        expected
    );
}
