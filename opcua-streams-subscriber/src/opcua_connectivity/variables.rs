use std::sync::{Arc, mpsc, Mutex};
use opcua_server::{
    prelude::*,
};
use opcua_streams_types::sensor_data::SensorData;
use std::str::FromStr;



pub fn add_variables(server: &mut Server, ns: u16, rx: mpsc::Receiver<String>, channel_address: String) {
    
    let address_space = server.address_space();

    let subscriber_node_id = NodeId::new(ns, "subscriber");
    let channel_node_id = NodeId::new(ns, "channel");
    let sensors_node_id = NodeId::new(ns, "sensors");  
    let gyroscope_node_id = NodeId::new(ns, "gyroscope");
    let magnetometer_node_id = NodeId::new(ns, "magnetometer");
    let temperature_node_id = NodeId::new(ns, "temperature");


    let rx = Arc::new(Mutex::new(rx));
    {
        let mut address_space = address_space.write().unwrap();
        
        ObjectBuilder::new(&subscriber_node_id, "subscriber", "Subscriber")
            .event_notifier(EventNotifier::SUBSCRIBE_TO_EVENTS)
            .organized_by(ObjectId::ObjectsFolder)
            .insert(&mut address_space);

        VariableBuilder::new(&channel_node_id, "channel", "Channel")
            .data_type(DataTypeId::String)
            .value(UAString::from(channel_address))
            .organized_by(subscriber_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .writable()
            .insert(&mut address_space);

        ObjectBuilder::new(&sensors_node_id, "sensors", "Sensors")
            .is_folder()
            .event_notifier(EventNotifier::SUBSCRIBE_TO_EVENTS)
            .organized_by(ObjectId::ObjectsFolder)
            .insert(&mut address_space);
            
        VariableBuilder::new(&gyroscope_node_id, "gyroscope", "Gyroscope")
            .data_type(DataTypeId::Int16)
            .value_rank(1)
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);

        VariableBuilder::new(&magnetometer_node_id, "magnetometer", "Magnetometer")
            .data_type(DataTypeId::Int16)
            .value_rank(1)
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);

        VariableBuilder::new(&temperature_node_id, "temperature", "Temperature")
            .data_type(DataTypeId::Int16)
            .organized_by(sensors_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .insert(&mut address_space);
        
    }

    {    
        server.add_polling_action(2000, move || {  
            let rx = rx.lock().unwrap();
            let mut address_space = address_space.write().unwrap();
               
            match rx.try_recv() {
                Ok(last_data) => {
                    let sensor_data: SensorData = serde_json::from_str(&last_data).unwrap();
                    for sensor in sensor_data.iot2tangle {
                        let node_id = NodeId::from_str(&sensor.sensor).unwrap();
                        println!("NodeId: {:?}", node_id);
                        let value: Variant = serde_json::from_value(sensor.data).unwrap();
                        let source_timestamp = serde_json::from_value(sensor_data.timestamp.clone()).unwrap();
                        let server_timestamp = DateTime::now().clone();
                        let _ = address_space.set_variable_value(node_id, value, &source_timestamp, &server_timestamp);
                    }
                }
                Err(_) => {}
            }             
        });
    }
}
