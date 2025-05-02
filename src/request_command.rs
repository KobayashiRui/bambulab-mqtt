use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RequestCommand {
    Info(Info),
    Print(Print),
}

impl RequestCommand {
    /// Serialize `self` into a pretty-printed JSON `String`.
    pub fn to_payload(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn get_sequence_id(&self) -> Option<String> {
        match self {
            RequestCommand::Info(info) => info.get_sequence_id(),
            RequestCommand::Print(print) => print.get_sequence_id(),
        }
    }
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Info {
    GetVersion(GetVersion)
}

impl Info {
    fn get_sequence_id(&self) -> Option<String> {
        match self {
            Info::GetVersion(get_version) => Some(get_version.sequence_id.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetVersion{
    sequence_id: String
}

impl GetVersion {
    pub fn new() -> Self{
        GetVersion { sequence_id: nanoid!()}
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
pub enum Print {
    ProjectFile(ProjectFile),
    Stop(Stop),
}

impl Print {
    fn get_sequence_id(&self) -> Option<String> {
        match self {
            Print::ProjectFile(project_file) => Some(project_file.sequence_id.clone()),
            Print::Stop(stop) => Some(stop.sequence_id.clone()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectFile{
    sequence_id: String,
    param: String,
    url: String,
    subtask_id: String,
    use_ams: bool,
    timelapse: bool,
    flow_cali: bool,
    bed_leveling: bool,
    layer_inspect: bool,
    vibration_cali: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stop{
    sequence_id: String,
    param: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;


    #[test]
    fn info_export_string() {
        let get_version = RequestCommand::Info(Info::GetVersion(GetVersion::new()));

        // シリアライズを行う
        let serialized = serde_json::to_string(&get_version).unwrap();
        // シリアライズした文字列を表示
        println!("Serialized: {}", serialized);

        // シリアライズを行う
        let payload = get_version.to_payload().unwrap();
        // シリアライズした文字列を表示
        println!("Payload: {}", payload);


        // serializedとpayloadは同じ内容であることを確認する
        assert_eq!(serialized, payload);
        
        // 期待される形式を確認する
        assert!(serialized.contains("\"command\":\"get_version\""));
        assert!(serialized.contains("\"sequence_id\":"));
    }

    #[test]
    fn info_get_sequence_id() {
        let get_version = RequestCommand::Info(Info::GetVersion(GetVersion::new()));
        let sequence_id = get_version.get_sequence_id().unwrap();
        println!("Sequence ID: {}", sequence_id);
        assert!(!sequence_id.is_empty());
    }

}
