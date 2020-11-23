// I2TH OPC UA Streams Gateway
// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2020 Hauke Jung

use opcua_streams_types::sensor_data::SensorData;
use opcua_streams_gateway::opcua_connectivity::opcua_client;
use opcua_streams_gateway::opcua_connectivity::opcua_server;
use opcua_streams_gateway::tangle_connectivity::tangle_publisher;
use std::sync::mpsc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let server_url = &args[1];

    let (tx_data, rx_data) = mpsc::channel::<SensorData>();
    let (tx_channel, rx_channel) = mpsc::channel::<String>();
    
    println!("Starting....");
    tangle_publisher::start(rx_data, tx_channel).await;
    opcua_client::start(server_url.clone(), tx_data).await;
    opcua_server::start(rx_channel).await;
}
