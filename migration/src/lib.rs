pub use sea_orm_migration::prelude::*;

pub mod m20250426_143220_create_profiles_table;
pub mod m20250426_151614_create_library_table;
pub mod m20250426_151715_create_media_table;
pub mod m20250426_151732_create_history_table;
pub mod m20250426_152510_create_media_metadata;
pub mod m20250426_152523_create_user_activities;
mod m20250426_153350_create_foreign_keys_migration;
mod m20250426_155425_create_peers_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250426_143220_create_profiles_table::Migration),
            Box::new(m20250426_151614_create_library_table::Migration),
            Box::new(m20250426_151715_create_media_table::Migration),
            Box::new(m20250426_151732_create_history_table::Migration),
            Box::new(m20250426_152510_create_media_metadata::Migration),
            Box::new(m20250426_152523_create_user_activities::Migration),
            Box::new(m20250426_153350_create_foreign_keys_migration::Migration),
            Box::new(m20250426_155425_create_peers_table::Migration),
        ]
    }
}
