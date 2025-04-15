use nanoid::nanoid;
use rumqttc::{Client, Connection, Event, Incoming, MqttOptions, Transport, LastWill, QoS, Packet};
use native_tls;

pub struct BambulabClient {
    client: Client,
    connection: Connection,
    report_topic: String,
    request_topic: String,
}

impl BambulabClient {
    pub fn connect(host: String, password: String, serial: String) -> Self{
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

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

