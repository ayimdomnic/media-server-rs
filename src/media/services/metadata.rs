use std::path::Path;
use tokio::fs;
use uuid::Uuid;

use crate::media::models::{MediaFile, MediaType};

pub struct MediaMetadataExtractor;

impl MediaMetadataExtractor {
    pub async fn extract(path: &Path) -> Option<MediaFile> {
        let file_name = path.file_name()?.to_str()?.to_string();
        let extension = path.extension()?.to_str()?.to_lowercase();

        let media_type = match extension.as_str() {
            "mp4" | "mkv" | "avi" | "mov" => MediaType::Video,
            "mp3" | "flac" | "wav" | "aac" => MediaType::Audio,
            "jpg" | "jpeg" | "png" | "gif" => MediaType::Image,
            _ => return None,
        };

        let metadata = fs::metadata(path).await.ok()?;

        Some(MediaFile {
            id: Uuid::new_v4(),
            library_id: Uuid::nil(),
            title: file_name,
            file_path: path.to_str()?.to_string(),
            media_type,
            size: metadata.len(),
            duration: None,
            bitrate: None,
            resolution: None,
        })
    }
}
