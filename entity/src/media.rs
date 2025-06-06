//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "media")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub library_id: Uuid,
    pub title: String,
    pub file_path: String,
    pub media_type: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::history::Entity")]
    History,
    #[sea_orm(
        belongs_to = "super::library::Entity",
        from = "Column::LibraryId",
        to = "super::library::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Library,
    #[sea_orm(has_many = "super::media_metadata::Entity")]
    MediaMetadata,
    #[sea_orm(has_many = "super::user_activity::Entity")]
    UserActivity,
}

impl Related<super::history::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::History.def()
    }
}

impl Related<super::library::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Library.def()
    }
}

impl Related<super::media_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MediaMetadata.def()
    }
}

impl Related<super::user_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserActivity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
