IRC Lib
=======
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
// let client = IrcClient::new(irc.subluminal.net:6667")
// client.build_message::<PrivMessage> -> MessageBuilder<T>
        // .params("Some message") 
        // .target(impl IrcTarget)
        // .channel("test12")
        // .execute()

// struct IrcMessage {
    // raw irc_rust::Message,
    // type: ircMessageType,
    // channel; IrcChannel,
    // user: IrcUser,
    //
// }

// struct IrcChannel {
    // name: String,
    // users: Vec<IrcUser>
// }

// struct IrcUser {
    // name: String,
    // permissions: IrcPermissionEnum (HALFOP/OP/etc)
// }