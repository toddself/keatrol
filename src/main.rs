use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct KeaConfig {
    dhcp4: Dhcp4Config
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Dhcp4Config {
    interfaces_config: InterfacesConfig
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct InterfacesConfig {
    interfaces: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ControlSocketConfig {
    socket_type: String,
    socket_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LeaseDatabaseConfig {
    type: LeaseDatabaseEnum
}



fn main() {
    println!("Hello, world!");
}
