# 🎍 BambuLab MQTT Crate
This project provides a Rust crate for sending and receiving data using the MQTT protocol with BambuLab devices.

## ✨ Features
- 🚀 Easy integration with BambuLab devices.
- 📡 Support for MQTT protocol.
- ⚡ Lightweight and efficient.

## 🛠️ Getting Started
To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
bambulab-mqtt = "0.1.0"
```

## 📚 Examples
Here are some examples to help you get started:

### 📦 Publish Data to MQTT
You can also publish data to an MQTT topic:

```rust
use bambulab_mqtt::mqtt_client;

fn main() {
    let client = mqtt_client::new("<host ip>", "<password>", "<serial>");

    let get_version = RequestCommand::Info(Info::GetVersion(GetVersion::new()));

    client.request(&get_version)
}
```

For more examples, check the `examples/` directory.

## 📜 License
This project is licensed under the Apache License 2.0. See the [LICENSE](./LICENSE) file for details.
