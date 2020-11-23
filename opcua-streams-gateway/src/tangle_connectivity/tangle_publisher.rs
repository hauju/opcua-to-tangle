use gateway_core::gateway::publisher::Channel;
use opcua_streams_types::config::Config;
use opcua_streams_types::sensor_data::SensorData;
use std::sync::{mpsc};
use std::{thread, time};
use std::fs::File;

pub async fn start(rx_data: mpsc::Receiver<SensorData>, tx_channel: mpsc::Sender<String>) {
    let config: Config = serde_json::from_reader(File::open("config.json").unwrap()).unwrap();
    
    let _ = thread::spawn(move || {
        let mut channel = Channel::new(
            config.node,
            config.mwm,
            config.local_pow,
            None, 
        );
    
        let (address, msg_id) = match channel.open() {
            Ok(a) => a,
            Err(_) => panic!("Could not connect to IOTA Node, try with another node!"),
        };   
        println!("Channel Address: {}", format!("{}:{}", address, msg_id));
        tx_channel.send(format!("{}:{}", address, msg_id)).unwrap();
        
        loop {
            match rx_data.try_recv() {
                Ok(sensor_data) => {
                    //let channel = channel.lock().unwrap();
                    match channel.write_signed(sensor_data) {
                        Ok(m) => {
                            println!("All good: {:?}", m);
                        }
                        Err(_e) => {
                            println!("This isn't working....");
                        }
                    };
                }
                Err(_) => {}
            }
            thread::sleep(time::Duration::from_secs(3));
        }
    });
}