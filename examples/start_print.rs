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

    let file_name = "3DBenchy by Creative Tool.gcode.3mf".to_string();
    let start_project = RequestCommand::Print(Print::ProjectFile(ProjectFile::new_simple(file_name)));
    let res = client.request(&start_project);
    match res {
        Ok(payload) => {
            println!("Payload: {}", payload);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}