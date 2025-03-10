mod test_plugins;

use std::rc::Rc;
use std::sync::mpsc::{Receiver, TryRecvError};

use irc_lib::{Client, IrcMessage};
pub use test_plugins::*;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::SyncRunner;
use testcontainers::{GenericImage, ImageExt};

const IRCD_PORT: u16 = 6667;

pub type TestResult = Result<(), Box<dyn std::error::Error>>;

pub struct TestHarness {
    container: Option<testcontainers::Container<GenericImage>>,
    clients: Vec<Rc<Client>>,
}

impl TestHarness {
    pub fn new() -> Self {
        TestHarness {
            container: None,
            clients: Vec::new(),
        }
    }
    pub fn start_ircd(&mut self) {
        panic_hook();
        let container = GenericImage::new("linuxserver/ngircd", "version-27-r0")
            .with_exposed_port(IRCD_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stdout("[ls.io-init] done."))
            .with_network("bridge")
            .start()
            .expect("Failed to start ircd");

        self.container = Some(container);
    }

    pub fn register_client(&mut self, client: Client) -> Rc<Client> {
        let item = Rc::new(client);
        self.clients.push(item.clone());
        item
    }

    pub fn get_host(&self) -> String {
        format!(
            "{}:{}",
            self.container.as_ref().unwrap().get_host().unwrap(),
            self.container
                .as_ref()
                .unwrap()
                .get_host_port_ipv4(IRCD_PORT.tcp())
                .unwrap()
        )
    }
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        while let Some(client) = self.clients.pop() {
            match Rc::try_unwrap(client) {
                Ok(client) => client.shutdown().unwrap(),
                Err(err) => println!("======== Could not shutdown client {:?} ========", err),
            }
        }
        // In case we couldn't do a graceful shutdown above... kill the container
        drop(self.container.take())
    }
}

pub fn panic_hook() {
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}

pub fn clear_buffer(receiver: &Receiver<IrcMessage>) {
    loop {
        let recv = receiver.try_recv();
        match recv {
            Ok(_) => (),
            Err(TryRecvError::Empty) => return,
            Err(TryRecvError::Disconnected) => {
                panic!("Receiver {:?} disconnected: {:?}", receiver, recv)
            }
        }
    }
}
