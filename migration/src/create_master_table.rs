use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Master::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Master::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Master::Name).string().not_null())
                    .col(ColumnDef::new(Master::Description).string())
                    .col(ColumnDef::new(Master::Password).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Master::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Master {
    Table,
    Id,
    Name,
    Description,
    Password,
}
