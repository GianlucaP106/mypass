pub use sea_orm_migration::prelude::*;

mod create_entry_table;
mod create_master_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(create_entry_table::Migration),
            Box::new(create_master_table::Migration),
        ]
    }
}
