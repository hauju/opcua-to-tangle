// I2TH OPC UA Streams Subscriber
// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2020 Hauke Jung

use std::sync::mpsc;
use std::{env, thread, time};
use std::fs::File;
use opcua_streams_subscriber::streams_subscriber::streams::Subscriber;
use opcua_streams_subscriber::opcua_connectivity::opcua_server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let channel_address = &args[1];
    let config: serde_json::Value = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    let node = config["node"].as_str().unwrap().to_string();
    let mut sub = Subscriber::new(node, channel_address.clone().to_string(), None);
    sub.channel_subscriber.connect().unwrap();
    println!("Connection to channel established successfully! \n Reading messages...");
    
    let (tx_data, rx_data) = mpsc::channel::<String>();

    let _ = thread::spawn(move || {
        // read old messages in channel
        let public_list = sub.read_all_public().unwrap();
        // listen for new messages sent to channel
        let mut public_list_len: usize = public_list.len();
        loop {
            let public_list = sub.read_all_public().unwrap();

            if &public_list.len() != &public_list_len.clone() {
                match public_list.last() {
                    Some(last_data) => {
                        //print out a pretty pretty JSON
                        println!("{} \n  \n", &last_data.replace("\\", ""));
                        tx_data.send(last_data.clone()).unwrap();
                    }
                    None => (),
                }
            }
            public_list_len = public_list.len().clone();
            
            // dont spam thee node with requests!
            thread::sleep(time::Duration::from_secs(5));
        }
    });  
    
    opcua_server::start(rx_data, channel_address.clone().to_string()).await;
}
