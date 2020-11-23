use std::sync::{Arc, mpsc, Mutex, RwLock};
use opcua_client::prelude::*;
use opcua_streams_types::sensor_data::SensorData;
use opcua_streams_types::sensor_type::SensorType;
use crate::timestamp_in_sec;
use std::thread;


pub async fn start(server_url: String, tx: mpsc::Sender<SensorData>) {

        opcua_console_logging::init();

        // Make the client configuration
        let mut client = ClientBuilder::new()
            .application_name("OPC UA Streams Gateway Client")
            .application_uri("urn:OpcUaStreamsGatewayClient")
            .pki_dir("./pki/client")
            .trust_server_certs(true)
            .create_sample_keypair(true)
            .session_retry_limit(3)
            .client().unwrap();

        println!("Connecting to {:?}", server_url);

        let _ = thread::spawn(move || {
            if let Ok(session) = client.connect_to_endpoint((&*server_url, SecurityPolicy::None.to_str(), MessageSecurityMode::None, UserTokenPolicy::anonymous()), IdentityToken::Anonymous) {
                if let Err(result) = subscribe_to_variables(session.clone(), 2, tx, server_url) {
                    println!("ERROR: Got an error while subscribing to variables - {}", result);
                } else {
                    // Loops forever. The publish thread will call the callback with changes on the variables
                    let _ = Session::run(session);
                }
            }
        }); 
}

fn subscribe_to_variables(session: Arc<RwLock<Session>>, ns: u16, tx: mpsc::Sender<SensorData>, url: String) -> Result<(), StatusCode> {
    let mut session = session.write().unwrap();
    let tx = Arc::new(Mutex::new(tx));
    // Creates a subscription with a data change callback
    let subscription_id = session.create_subscription(20000.0, 10, 30, 0, 0, true, DataChangeCallback::new(move |changed_monitored_items| {      
        println!("Data change from server:");
        let mut my_vec = Vec::new();
        changed_monitored_items.iter().for_each(|item| {
            my_vec.push(get_sensor_type(item));
        });
   
        let data = SensorData {iot2tangle: my_vec, device: url.to_string(), timestamp: serde_json::Value::from(timestamp_in_sec())};
        //let xs = serde_json::to_string(&data).unwrap();
        println!("Serialize: {:?}", data);
        let tx = tx.lock().unwrap();
        //tx.send(MyData::Data(data)).unwrap();
        tx.send(data).unwrap();
    }))?;
    println!("Created a subscription with id = {}", subscription_id);

    // Create some monitored items
    let items_to_create: Vec<MonitoredItemCreateRequest> = ["gyroscope", "magnetometer", "temperature"].iter()
        .map(|v| NodeId::new(ns, *v).into()).collect();
    let _ = session.create_monitored_items(subscription_id, TimestampsToReturn::Both, &items_to_create)?;

    Ok(())
}

fn get_sensor_type(item: &MonitoredItem) -> SensorType {
    let node_id = item.item_to_monitor().node_id.clone().to_string();
    //let data_value = item.value().clone();
    let data_value = item.value().clone().value.unwrap();
    let data = serde_json::to_value(data_value).unwrap();
    println!("{}", data);
    return SensorType { sensor: node_id, data: data };
}