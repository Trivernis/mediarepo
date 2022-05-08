use crate::async_std::task::Task;
use crate::sea_orm::tests_cfg::cake_filling::PrimaryKey;
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(create_content_descriptors()).await?;
        manager.create_table(create_files()).await?;
        manager.create_table(create_file_metadata()).await?;
        manager.create_table(create_namespaces()).await?;
        manager.create_table(create_tags()).await?;
        manager.create_table(create_cd_tag_mappings()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        todo!()
    }
}

fn create_file_metadata() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(FileMetadata::Table)
        .col(
            ColumnDef::new(FileMetadata::FileId)
                .big_integer()
                .not_null()
                .primary_key(),
        )
        .col(ColumnDef::new(FileMetadata::Size).integer().not_null())
        .col(ColumnDef::new(FileMetadata::Name).string_len(128))
        .col(ColumnDef::new(FileMetadata::Comment).string_len(1024))
        .col(
            ColumnDef::new(FileMetadata::ImportTime)
                .date_time()
                .not_null(),
        )
        .col(
            ColumnDef::new(FileMetadata::CreationTime)
                .date_time()
                .not_null(),
        )
        .col(
            ColumnDef::new(FileMetadata::ChangeTime)
                .date_time()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(FileMetadata::Table, FileMetadata::FileId)
                .to(Files::Table, Files::Id),
        )
        .to_owned()
}

fn create_files() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(Files::Table)
        .col(
            ColumnDef::new(Files::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(Files::Status)
                .integer()
                .default(10)
                .not_null(),
        )
        .col(ColumnDef::new(Files::CdId).big_integer().not_null())
        .col(
            ColumnDef::new(Files::MimeType)
                .string_len(128)
                .default("application/octet-stream")
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(Files::Table, Files::CdId)
                .to(ContentDescriptors::Table, ContentDescriptors::Id),
        )
        .index(Index::create().table(Files::Table).col(Files::CdId))
        .to_owned()
}

fn create_content_descriptors() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(ContentDescriptors::Table)
        .col(
            ColumnDef::new(ContentDescriptors::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(ContentDescriptors::Descriptor)
                .binary()
                .unique_key(),
        )
        .to_owned()
}

fn create_tags() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(Tags::Table)
        .col(
            ColumnDef::new(Tags::Id)
                .integer()
                .primary_key()
                .auto_increment(),
        )
        .col(ColumnDef::new(Tags::NamespaceId).big_integer())
        .col(ColumnDef::new(Tags::Name).string_len(128).not_null())
        .foreign_key(
            ForeignKey::create()
                .from(Tags::Table, Tags::NamespaceId)
                .to(Namespaces::Table, Namespaces::Id),
        )
        .index(
            Index::create()
                .table(Tags::Table)
                .col(Tags::Name)
                .full_text(),
        )
        .index(
            Index::create()
                .unique()
                .table(Tags::Table)
                .col(Tags::NamespaceId)
                .col(Tags::Name),
        )
        .to_owned()
}

fn create_cd_tag_mappings() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(CdTagMappings::Table)
        .col(ColumnDef::new(CdTagMappings::CdId).big_integer().not_null())
        .col(
            ColumnDef::new(CdTagMappings::TagId)
                .big_integer()
                .not_null(),
        )
        .primary_key(
            Index::create()
                .table(CdTagMappings::TagId)
                .col(CdTagMappings::CdId)
                .col(CdTagMappings::TagId),
        )
        .foreign_key(
            ForeignKey::create()
                .from(CdTagMappings::Table, CdTagMappings::CdId)
                .to(ContentDescriptors::Table, ContentDescriptors::Id),
        )
        .foreign_key(
            ForeignKey::create()
                .from(CdTagMappings::Table, CdTagMappings::TagId)
                .to(Tags::Table, Tags::Id),
        )
        .index(
            Index::create()
                .table(CdTagMappings::Table)
                .col(CdTagMappings::TagId),
        )
        .to_owned()
}

fn create_namespaces() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(Namespaces::Table)
        .col(
            ColumnDef::new(Namespaces::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(ColumnDef::new(Namespaces::Name).string_len(128).not_null())
        .index(
            Index::create()
                .unique()
                .table(Namespaces::Table)
                .col(Namespaces::Name)
                .full_text(),
        )
        .to_owned()
}

#[derive(Iden)]
enum FileMetadata {
    Table,
    FileId,
    Size,
    Name,
    Comment,
    ImportTime,
    CreationTime,
    ChangeTime,
}

#[derive(Iden)]
enum Files {
    Table,
    Id,
    Status,
    CdId,
    MimeType,
}

#[derive(Iden)]
enum ContentDescriptors {
    Table,
    Id,
    Descriptor,
}

#[derive(Iden)]
enum Tags {
    Table,
    Id,
    NamespaceId,
    Name,
}

#[derive(Iden)]
enum CdTagMappings {
    Table,
    CdId,
    TagId,
}

#[derive(Iden)]
enum Namespaces {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Sources {
    Table,
    Id,
    Url,
}

#[derive(Iden)]
enum CdSourceMapping {
    Table,
    CdId,
    SourceId,
}

#[derive(Iden)]
enum SortingPreset {
    Table,
    Id,
}

#[derive(Iden)]
enum SortKeys {
    Table,
    KeyType,
    Ascending,
    Value,
}

#[derive(Iden)]
enum SortingPresetKey {
    Table,
    PresetId,
    KeyId,
    KeyIndex,
}

#[derive(Iden)]
enum JobStates {
    Table,
    JobType,
    Value,
}
