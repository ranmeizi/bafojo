pub use sea_orm_migration::prelude::*;

mod utils;
mod m20230426_022644_create_table;
mod m20230904_023028_init_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230426_022644_create_table::Migration),
            Box::new(m20230904_023028_init_data::Migration),
        ]
    }
}
