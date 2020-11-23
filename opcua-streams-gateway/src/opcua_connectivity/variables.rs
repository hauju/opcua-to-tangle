use std::sync::{Arc, mpsc, Mutex};
use opcua_server::{
    prelude::*,
};

pub fn add_variables(server: &mut Server, ns: u16, rx: mpsc::Receiver<String>) {
    
    let address_space = server.address_space();
    
    // Node IDs
    let author_node_id = NodeId::new(ns, "author");
    let channel_node_id = NodeId::new(ns, "channel");
    let address_node_id = NodeId::new(ns, "address");
    let msg_id_node_id = NodeId::new(ns, "msg_id");
    
    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write().unwrap();

        ObjectBuilder::new(&author_node_id, "author", "Author")
            .event_notifier(EventNotifier::SUBSCRIBE_TO_EVENTS)
            .organized_by(ObjectId::ObjectsFolder)
            .insert(&mut address_space);

        VariableBuilder::new(&channel_node_id, "channel", "Channel")
            .data_type(DataTypeId::String)
            .organized_by(author_node_id.clone())
            .has_type_definition(VariableTypeId::BaseDataVariableType)
            .description(LocalizedText::from("To read the messages copy the channel root into https://explorer.iot2tangle.io/"))
            .insert(&mut address_space);

        VariableBuilder::new(&address_node_id, "address", "Address")
            .data_type(DataTypeId::String)
            .property_of(channel_node_id.clone())
            .has_type_definition(VariableTypeId::PropertyType)
            .has_modelling_rule(ObjectId::ModellingRule_Mandatory)
            .insert(&mut address_space);

        VariableBuilder::new(&msg_id_node_id, "msg_id", "Message ID")
            .data_type(DataTypeId::String)
            .property_of(channel_node_id.clone())
            .has_type_definition(VariableTypeId::PropertyType)
            .has_modelling_rule(ObjectId::ModellingRule_Mandatory)
            .insert(&mut address_space);

    }

    {
        let rx = Arc::new(Mutex::new(rx));
        server.add_polling_action(1000, move || {
            let mut address_space = address_space.write().unwrap();
            let rx = rx.lock().unwrap();
            let now = DateTime::now();
            match rx.try_recv() {
                Ok(channel_address) => {
                    let _ = address_space.set_variable_value(channel_node_id.clone(), channel_address.clone(), &now, &now);
                    let str_iter = channel_address.split(":").collect::<Vec<&str>>();
                    let address = str_iter[0];
                    let msg_id = str_iter[1];
                    let _ = address_space.set_variable_value(address_node_id.clone(), address, &now, &now);
                    let _ = address_space.set_variable_value(msg_id_node_id.clone(), msg_id, &now, &now);
                }
                Err(_) => {}
            };         
        });
    }
}