use serde::{Deserialize, Serialize};
use strum_macros::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaFile {
    pub id: Uuid,
    pub library_id: Uuid,
    pub title: String,
    pub file_path: String,
    pub media_type: MediaType,
    pub size: u64,
    pub duration: Option<f64>,      // in seconds
    pub bitrate: Option<u32>,       // in kbps
    pub resolution: Option<String>, // e.g., "1920x1080"
}

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
pub enum MediaType {
    Video,
    Audio,
    Image,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamRequest {
    pub media_id: Uuid,
    pub profile_id: Uuid,
    pub seek_position: Option<f64>, // in seconds
    pub prefer_p2p: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StreamResponse {
    pub stream_type: StreamType,
    pub url: String,
    pub p2p_peers: Vec<P2PPeer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StreamType {
    P2P,
    HTTP,
    HLS,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct P2PPeer {
    pub peer_id: String,
    pub ip_address: String,
    pub port: u16,
    pub has_full_file: bool,
}
