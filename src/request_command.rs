use serde::{Deserialize, Serialize};
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum RequestCommand {
    Info(Info),
    Print(Print),
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
enum Info {
    GetVersion(GetVersion)
}

#[derive(Serialize, Deserialize, Debug)]
struct GetVersion{
    sequence_id: String
}

impl GetVersion {
    pub fn new() -> Self{
        GetVersion { sequence_id: nanoid!()}
    }
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "command", rename_all = "snake_case")]
enum Print {
    ProjectFile(ProjectFile),
    Stop(Stop),
}

#[derive(Serialize, Deserialize, Debug)]
struct ProjectFile{
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
struct Stop{
    sequence_id: String,
    param: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn info_export_string() {
        let get_version = RequestCommand::Info(Info::GetVersion(GetVersion::new()));
        
        // シリアライズを行う
        let serialized = serde_json::to_string(&get_version).unwrap();
        
        // シリアライズした文字列を表示
        println!("Serialized: {}", serialized);
        
        // 期待される形式を確認する
        assert!(serialized.contains("\"command\":\"get_version\""));
        assert!(serialized.contains("\"sequence_id\":"));
    }
}
