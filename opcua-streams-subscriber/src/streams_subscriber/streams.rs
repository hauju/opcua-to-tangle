use crate::streams_subscriber::subscriber::Channel;

pub struct Subscriber {
    pub channel_subscriber: Channel,
}

impl Subscriber {
    pub fn new(node: String, channel_address: String, seed: Option<String>) -> Self {
        let str_iter = channel_address.split(":").collect::<Vec<&str>>();
        let address = str_iter[0];
        let msg_id = str_iter[1];
        let subscriber: Channel = Channel::new(node, address.to_string(), msg_id.to_string(), seed);
        Self {
            channel_subscriber: subscriber,
        }
    }

    /// Derives Msg Ids for channel and reads messages associated with them,
    /// returns an empty vector if no now messages where found
    ///
    pub fn read_all_public(&mut self) -> serde_json::Result<Vec<String>> {
        let tag_list = self.channel_subscriber.get_next_message().unwrap();

        let mut msg_list: Vec<String> = vec![];
        for signed_message_tag in tag_list {
            let msgs: Vec<(Option<String>, Option<String>)> = self
                .channel_subscriber
                .read_signed(signed_message_tag)
                .unwrap();
            for (msg_p, _msg_m) in msgs {
                match msg_p {
                    None => continue,
                    Some(message) => msg_list.push(message),
                }
            }
        }

        Ok(msg_list)
    }
}



