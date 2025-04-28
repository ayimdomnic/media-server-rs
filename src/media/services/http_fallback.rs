use sea_orm::EntityTrait;
use std::path::Path;
use tokio::{fs::File, io::AsyncReadExt};
use uuid::Uuid;

use crate::{errors::AppError, state::AppState};

pub struct HttpStreamer;

impl HttpStreamer {
    pub async fn stream_file(
        state: AppState,
        media_id: Uuid,
        range_header: Option<String>,
    ) -> Result<(Vec<u8>, String, Option<String>), AppError> {
        let media = entity::media::Entity::find_by_id(media_id)
            .one(&state.conn)
            .await?
            .ok_or(AppError::NotFound)?;

        let path = Path::new(&media.file_path);
        let mut file = File::open(path).await.map_err(|_| AppError::NotFound)?;

        // Handle range requests for seeking
        let (buffer, content_range) = if let Some(range) = range_header {
            Self::handle_range_request(&mut file, &range).await?
        } else {
            let mut buf = Vec::new();
            let _ = file
                .read_to_end(&mut buf)
                .await
                .map_err(|_| AppError::InternalServerError);
            (buf, None)
        };

        let content_type = match media.media_type.as_str() {
            "video" => "video/mp4",
            "audio" => "audio/mpeg",
            _ => "application/octet-stream",
        };

        Ok((buffer, content_type.to_string(), content_range))
    }

    async fn handle_range_request(
        file: &mut File,
        range_header: &str,
    ) -> Result<(Vec<u8>, Option<String>), AppError> {
        let _ = file;
        let _ = range_header;
        // Implement range request handling
        Ok((vec![], None))
    }
}
