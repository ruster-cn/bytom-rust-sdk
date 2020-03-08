use serde::Deserialize;
use serde::Serialize;

#[derive(Default,Clone, Debug, Deserialize, Serialize)]
pub struct NetInfo{
    pub listening: bool,
    pub syncing: bool,
    pub mining: bool,
    pub peer_count: u8,
    pub current_block: u64,
    pub highest_block: u64,
    pub network_id: String,
    pub version_info: VersionInfo,
}

#[derive(Default,Clone, Debug, Deserialize, Serialize)]
pub struct VersionInfo{
    pub version: String,
    pub update:u16,
    pub new_version:String,
}

#[derive(Default,Clone, Debug, Deserialize, Serialize)]
pub struct PeerInfo{
    pub peer_id: String,
    pub remote_addr: String,
    pub height: u64,
    pub ping: String,
    pub duration:String,
    pub total_sent: u64,
    pub total_received: u64,
    pub average_sent_rate: u64,
    pub average_received_rate: u64,
    pub current_sent_rate: u64,
    pub current_received_rate: u64,
}