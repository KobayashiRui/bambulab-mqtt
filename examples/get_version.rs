use std::env;
use bambulab_mqtt::BambulabClient;
use bambulab_mqtt::request_command::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let host = args.get(1).expect("Please provide a host as the first argument.");
    let password = args.get(2).expect("Please provide a password as the second argument.");
    let serial = args.get(3).expect("Please provide a serial as the third argument.");
    println!("Host: {}", host);
    println!("Password: {}", password);
    println!("Serial: {}", serial);

    let mut client = BambulabClient::new(host.to_string(), password.to_string(), serial.to_string());

    let get_version = RequestCommand::Info(Info::GetVersion(GetVersion::new()));
    let res = client.request(&get_version);
    match res {
        Ok(payload) => {
            println!("Payload: {}", payload);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}