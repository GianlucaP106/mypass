use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entry::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Entry::Id).uuid().primary_key().not_null())
                    .col(ColumnDef::new(Entry::Name).string().not_null())
                    .col(ColumnDef::new(Entry::Description).string())
                    .col(ColumnDef::new(Entry::Username).string())
                    .col(ColumnDef::new(Entry::Password).string().not_null())
                    .col(ColumnDef::new(Entry::Url).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entry::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Entry {
    Table,
    Id,
    Name,
    Description,
    Username,
    Password,
    Url,
}
