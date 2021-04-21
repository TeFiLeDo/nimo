#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Data {
    pub server_id: u64,
    pub server_name: String,
    pub server_country: String,
    pub server_host: String,
    pub isp: String,
    pub download: u64,
    pub upload: u64,
}
