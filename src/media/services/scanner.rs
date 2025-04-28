use crate::{errors::AppError, media::models, state::AppState};

use super::metadata::MediaMetadataExtractor;
use chrono::Utc;
use entity::{self};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use std::path::Path;
use uuid::Uuid;
use walkdir::WalkDir;

pub struct MediaScanner;

impl MediaScanner {
    pub async fn scan_library(
        state: AppState,
        library_id: Uuid,
        library_path: String,
    ) -> Result<(), AppError> {
        let _db = &state.conn;
        let path = Path::new(&library_path);

        // Clean orphaned media first
        Self::clean_orphaned_media(state.clone(), library_id, library_path.clone()).await?;

        // Process files
        for entry in WalkDir::new(path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Some(metadata) = MediaMetadataExtractor::extract(entry.path()).await {
                    Self::process_media_file(state.clone(), library_id, entry.path(), metadata)
                        .await?;
                }
            }
        }

        Ok(())
    }

    async fn clean_orphaned_media(
        state: AppState,
        library_id: Uuid,
        library_path: String,
    ) -> Result<(), AppError> {
        let db = &state.conn;
        let all_media = entity::media::Entity::find()
            .filter(entity::media::Column::LibraryId.eq(library_id))
            .all(db)
            .await?;

        for media_item in all_media {
            let full_path = Path::new(&library_path).join(&media_item.file_path);
            if !full_path.exists() {
                entity::media::Entity::delete_by_id(media_item.id)
                    .exec(db)
                    .await?;
            }
        }

        Ok(())
    }

    async fn process_media_file(
        state: AppState,
        library_id: Uuid,
        path: &Path,
        metadata: models::MediaFile,
    ) -> Result<(), AppError> {
        let db = &state.conn;
        let file_path = path
            .to_str()
            .ok_or(AppError::ValidationError("Invalid file path".into()))?;

        let existing_media = entity::media::Entity::find()
            .filter(entity::media::Column::LibraryId.eq(library_id))
            .filter(entity::media::Column::FilePath.eq(file_path))
            .one(db)
            .await?;

        if existing_media.is_none() {
            let new_media = entity::media::ActiveModel {
                id: Set(metadata.id),
                library_id: Set(library_id),
                title: Set(metadata.title),
                file_path: Set(file_path.to_string()),
                media_type: Set(metadata.media_type.to_string()),
                created_at: Set(Utc::now().naive_utc()),
                updated_at: Set(Utc::now().naive_utc()),
            };

            new_media.insert(db).await?;
        }

        Ok(())
    }
}
