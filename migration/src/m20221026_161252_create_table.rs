use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Object::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Object::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Object::ObjectName).string().not_null())
                    .col(ColumnDef::new(Object::ObjectType).string().not_null())
                    .col(ColumnDef::new(Object::ObjectSize).big_integer().not_null())
                    .col(
                        ColumnDef::new(Object::ObjectDescription)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Object::ObjectBucketName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Object::BlockId).integer().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Block::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Block::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Block::BlockName)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Block::BlockUid)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Block::BlockDescription).string().not_null())
                    .col(ColumnDef::new(Block::BlockType).string().not_null())
                    .col(ColumnDef::new(Block::BlockBucketPath).string().not_null())
                    .col(ColumnDef::new(Block::BlockFormat).string().not_null())
                    .col(
                        ColumnDef::new(Block::BlockFields)
                            .array(ColumnType::String(Some(32)))
                            .not_null(),
                    )
                    .col(ColumnDef::new(Block::CreateAt).date().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Object::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Block::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Block {
    Table,
    Id,
    BlockName,
    BlockUid,
    BlockDescription,
    BlockType,
    BlockBucketPath,
    BlockFormat,
    BlockFields,
    CreateAt,
}

#[derive(Iden)]
enum Object {
    Table,
    Id,
    ObjectName,
    ObjectType,
    ObjectSize,
    ObjectDescription,
    ObjectBucketName,
    BlockId,
}
