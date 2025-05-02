use serde_json::Value;
use log::info;

use nanoid::nanoid;
use rumqttc::{Client, Connection, Event, Incoming, LastWill, MqttOptions, Outgoing, Packet, QoS, Transport};
use native_tls;

pub mod request_command;

use request_command::RequestCommand;

pub struct BambulabClient {
    client: Client,
    connection: Connection,
    report_topic: String,
    request_topic: String,
}

// sequence_idの抽出
fn extract_sequence_id(payload: &str) -> Option<String>{
    let json: serde_json::Value = serde_json::from_str(payload).unwrap();
    if let Value::Object(map) = json {
        for (_key, inner) in map {
            if let Some(sequence_id) = inner.get("sequence_id") {
                if let Some(sequence_id_str) = sequence_id.as_str() {
                    return Some(sequence_id_str.to_string());
                }
            }
        }
    }
    None
}

fn check_sequence_id(payload: &str, sequence_id: &String) -> bool {
    match extract_sequence_id(payload) {
        Some(get_sequence_id) => {
            if *sequence_id == get_sequence_id {
                return true;
            } else {
                return false;
            }
        },
        None => {
            return false;
        }
    }
}

impl BambulabClient {
    pub fn new(host: String, password: String, serial: String) -> Self{
        let connector = native_tls::TlsConnector::builder()
                                    .danger_accept_invalid_certs(true)
                                    .danger_accept_invalid_hostnames(true)
                                    .build().unwrap();
        let mut mqttoptions: MqttOptions = MqttOptions::new(nanoid!(), host, 8883);
        mqttoptions.set_credentials("bblp", password);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));
        mqttoptions.set_transport(Transport::tls_with_config(connector.into()));

        let report_topic = format!("device/{}/report", &serial);
        let request_topic= format!("device/{}/request", &serial);

        let (client, connection) = Client::new(mqttoptions, 10);

        BambulabClient { 
            report_topic,
            request_topic,
            client,
            connection,
        }
    }


    fn wait_request(&mut self, sequence_id:&String) {
        for (i, notification) in self.connection.iter().enumerate() {
            match notification {
                Ok(Event::Incoming(packet)) => {
                    info!("Incoming {:?}",packet);
                    // もし Publish パケットなら、トピックとペイロードを取り出してみる
                    if let Packet::Publish(p) = packet {
                        info!("    → Publish received on '{}': {}", p.topic, String::from_utf8_lossy(&p.payload));
                        if check_sequence_id(&String::from_utf8_lossy(&p.payload), &sequence_id) {
                            info!("    → Sequence ID matched: {}", sequence_id);
                            break;
                        }
                    }
                }
                Ok(notif) => {
                    info!("{i}. Notification = {notif:?}");
                }
                Err(error) => {
                    info!("{i}. Notification = {error:?}");
                }
            }
        }
    }

    pub fn request(&mut self, cmd: &RequestCommand) -> Result<(), serde_json::Error> {
        match cmd.to_payload() {
            Ok(payload) => {
                info!("Publishing over MQTT:\n{}", payload);
                let sequence_id = cmd.get_sequence_id().unwrap();
                self.client.subscribe(&self.report_topic, QoS::AtMostOnce).unwrap();
                self.client.publish(&self.request_topic, QoS::AtLeastOnce, true, payload).unwrap();
                self.wait_request(&sequence_id);
                self.client.unsubscribe(&self.report_topic).unwrap();
                // TODO: wait publish result with timeout
                Ok(())
            }
            Err(e) =>{
                Err(e)
            }
        }
    }

    pub fn disconnect(&mut self) {
        self.client.disconnect().unwrap();
    }

}

