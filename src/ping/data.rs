use std::net::IpAddr;

pub type Data = Vec<DataEntry>;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct DataEntry {
    pub target_name: String,
    pub target_address: IpAddr,
    pub sent: u8,
    pub received: u8,
}
