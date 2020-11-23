// I2TH OPC UA Sensors Server
// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2020 Hauke Jung

//! This is a simple server for OPC UA. It reveals some OPC UA nodes we can connect to. 
//! The node values are set every second by random values.
use std::path::PathBuf;
use rand::Rng;
use opcua_server::prelude::*;

fn main() {
    // This enables logging via env_logger & log crate macros. If you don't need logging or want
    // to implement your own, omit this line.
    opcua_console_logging::init();

    // Create an OPC UA server with sample configuration and default node set
    let mut server = Server::new(ServerConfig::load(&PathBuf::from("server.conf")).unwrap());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write().unwrap();
        address_space.register_namespace("urn:opcua-sensors-server").unwrap()
    };

    // Add some variables
    add_sensor_variables(&mut server, ns);

    // Run the server. This does not ordinarily exit so you must Ctrl+C to terminate
    println!("Starting....");
    server.run();
}

/// Creates some sample variables, and some push / pull examples that update them
fn add_sensor_variables(server: &mut Server, ns: u16) {
    // These will be the node ids of the variables
    let sensors_node_id = NodeId::new(ns, "sensors");  
    let gyroscope_node_id = NodeId::new(ns, "gyroscope");
    let magnetometer_node_id = NodeId::new(ns, "magnetometer");
    let temperature_node_id = NodeId::new(ns, "temperature");

    let address_space = server.address_space();

    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write().unwrap();

        ObjectBuilder::new(&sensors_node_id, "sensors", "Sensors")
            .is_folder()
            .event_notifier(EventNotifier::SUBSCRIBE_TO_EVENTS)
            .organized_by(ObjectId::ObjectsFolder)
            .insert(&mut address_space);
            
        VariableBuilder::new(&gyroscope_node_id, "gyroscope", "Gyroscope")
            .data_type(DataTypeId::Int16)
            .value_rank(1)
            .writable()
            .value(vec![13i16, 37i16, -42i16])
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);

        VariableBuilder::new(&magnetometer_node_id, "magnetometer", "Magnetometer")
            .data_type(DataTypeId::Int16)
            .value_rank(1)
            .writable()
            .value(vec![0i16, 0i16, 0i16])
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);

        VariableBuilder::new(&temperature_node_id, "temperature", "Temperature")
            .data_type(DataTypeId::Int16)
            .value(0)
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);
    }

    {
        // Store a counter and a flag in a tuple
        server.add_polling_action(1000, move || {
            let mut address_space = address_space.write().unwrap();
            let now = DateTime::now();
            
            // Generate random values
            let mut rng = rand::thread_rng();
            let gyroscope_value: Vec<i16> = (0..3).map(|_| { rng.gen_range(-1000, 1000)}).collect();
            let magnetometer_value: Vec<i16> = (0..3).map(|_| { rng.gen_range(-100, 100)}).collect();
            let temperature_value = rng.gen_range(-20, 60);

            let _ = address_space.set_variable_value(gyroscope_node_id.clone(), gyroscope_value, &now, &now);
            let _ = address_space.set_variable_value(magnetometer_node_id.clone(), magnetometer_value, &now, &now);
            let _ = address_space.set_variable_value(temperature_node_id.clone(), temperature_value, &now, &now);
                   
        });
    }
}