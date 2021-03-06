use std::path::PathBuf;
use std::sync::{mpsc};
use opcua_server::prelude::*;

use crate::opcua_connectivity::variables::add_variables;

pub async fn start(rx_channel: mpsc::Receiver<String>, channel_address: String) {

    let mut server = Server::new(ServerConfig::load(&PathBuf::from("server.conf")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write().unwrap();
        address_space.register_namespace("urn:opcua-streams-server").unwrap()
    };

    // Add some variables of our own
    add_variables(&mut server, ns, rx_channel, channel_address);

    println!("Starting OPC UA Server");
    // Run the server. This does not ordinarily exit so you must Ctrl+C to terminate
    server.run();
}