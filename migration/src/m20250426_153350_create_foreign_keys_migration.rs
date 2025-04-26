use sea_orm_migration::prelude::*;

use crate::{
    m20250426_143220_create_profiles_table::Profile,
    m20250426_151614_create_library_table::Library, m20250426_151715_create_media_table::Media,
    m20250426_151732_create_history_table::History,
    m20250426_152510_create_media_metadata::MediaMetadata,
    m20250426_152523_create_user_activities::UserActivity,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-history-profile_id")
                    .from(History::Table, History::ProfileId)
                    .to(Profile::Table, Profile::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-history-media_id")
                    .from(History::Table, History::MediaId)
                    .to(Media::Table, Media::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Media Table Foreign Key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-media-library_id")
                    .from(Media::Table, Media::LibraryId)
                    .to(Library::Table, Library::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // Media Metadata Table Foreign Key
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-metadata-media_id")
                    .from(MediaMetadata::Table, MediaMetadata::MediaId)
                    .to(Media::Table, Media::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // User Activity Table Foreign Keys
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_activity-profile_id")
                    .from(UserActivity::Table, UserActivity::ProfileId)
                    .to(Profile::Table, Profile::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-user_activity-media_id")
                    .from(UserActivity::Table, UserActivity::MediaId)
                    .to(Media::Table, Media::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserActivity::Table)
                    .name("fk-user_activity-media_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(UserActivity::Table)
                    .name("fk-user_activity-profile_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(MediaMetadata::Table)
                    .name("fk-metadata-media_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Media::Table)
                    .name("fk-media-library_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(History::Table)
                    .name("fk-history-media_id")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(History::Table)
                    .name("fk-history-profile_id")
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
