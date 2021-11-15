use std::time::Duration;

use kafka::producer::{Producer, Record, RequiredAcks};

pub struct Publisher<'a> {
    location: &'a str,
    topic: &'a str,
}

impl<'a> Publisher<'a> {
    pub fn new(location: &'a str, topic: &'a str) -> Publisher<'a> {
        Publisher {
            location: location,
            topic: topic,
        }
    }

    pub fn publish(&self, info: String) {
        let producer = Producer::from_hosts(vec![self.location.to_owned()])
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create();

        match producer {
            Ok(mut producer) => {
                let data = info.as_bytes();

                if let Err(e) = producer.send(&Record {
                    topic: self.topic,
                    partition: -1,
                    key: (),
                    value: data,
                }) {
                    println!("Failed producing messages: {}", e);
                }

                if let Err(e) = producer.send(&Record::from_value(self.topic, data)) {
                    println!("Failed producing messages: {}", e);
                }
            }

            Err(e) => {
                println!("Failed producing messages: {}", e);
            }
        }
    }
}
