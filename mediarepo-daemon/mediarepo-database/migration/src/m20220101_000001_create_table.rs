use crate::drop_tables;
use sea_orm_migration::prelude::*;

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
        manager.create_table(create_sources()).await?;
        manager.create_table(create_cd_sources_mappings()).await?;
        manager.create_table(create_sorting_presets()).await?;
        manager.create_table(create_sort_keys()).await?;
        manager.create_table(create_sorting_preset_key()).await?;
        manager.create_table(create_job_states()).await?;

        manager
            .create_index(
                Index::create()
                    .name("index_files_cd_id")
                    .table(Files::Table)
                    .col(Files::CdId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_tags_name")
                    .table(Tags::Table)
                    .col(Tags::Name)
                    .full_text()
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_cd_tag_mappings_tag_id")
                    .table(CdTagMappings::Table)
                    .col(CdTagMappings::TagId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        drop_tables!(
            manager,
            ContentDescriptors::Table,
            Files::Table,
            FileMetadata::Table,
            Namespaces::Table,
            Tags::Table,
            CdTagMappings::Table,
            Sources::Table,
            CdSourceMappings::Table,
            SortingPresets::Table,
            SortKeys::Table,
            SortingPresetKeys::Table,
            JobStates::Table
        );

        Ok(())
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
                .to(Files::Table, Files::Id)
                .on_delete(ForeignKeyAction::Cascade),
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
                .to(ContentDescriptors::Table, ContentDescriptors::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
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
                .to(ContentDescriptors::Table, ContentDescriptors::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .from(CdTagMappings::Table, CdTagMappings::TagId)
                .to(Tags::Table, Tags::Id)
                .on_delete(ForeignKeyAction::Cascade),
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

fn create_sources() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(Sources::Table)
        .col(
            ColumnDef::new(Sources::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(ColumnDef::new(Sources::Url).string_len(512).not_null())
        .index(
            Index::create()
                .unique()
                .table(Sources::Table)
                .col(Sources::Url)
                .full_text(),
        )
        .to_owned()
}

fn create_cd_sources_mappings() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(CdSourceMappings::Table)
        .col(
            ColumnDef::new(CdSourceMappings::CdId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(CdSourceMappings::SourceId)
                .big_integer()
                .not_null(),
        )
        .foreign_key(
            ForeignKey::create()
                .from(CdSourceMappings::Table, CdSourceMappings::CdId)
                .to(ContentDescriptors::Table, ContentDescriptors::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .from(CdSourceMappings::Table, CdSourceMappings::SourceId)
                .to(Sources::Table, Sources::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .primary_key(
            Index::create()
                .table(CdSourceMappings::Table)
                .col(CdSourceMappings::CdId)
                .col(CdSourceMappings::SourceId)
                .full_text(),
        )
        .to_owned()
}

fn create_sorting_presets() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(SortingPresets::Table)
        .col(
            ColumnDef::new(SortingPresets::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .to_owned()
}

fn create_sort_keys() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(SortKeys::Table)
        .col(
            ColumnDef::new(SortKeys::Id)
                .big_integer()
                .primary_key()
                .auto_increment(),
        )
        .col(
            ColumnDef::new(SortKeys::KeyType)
                .integer()
                .not_null()
                .default(0),
        )
        .col(
            ColumnDef::new(SortKeys::Ascending)
                .boolean()
                .not_null()
                .default(false),
        )
        .col(ColumnDef::new(SortKeys::Value).string_len(128))
        .to_owned()
}

fn create_sorting_preset_key() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(SortingPresetKeys::Table)
        .col(
            ColumnDef::new(SortingPresetKeys::PresetId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(SortingPresetKeys::KeyId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(SortingPresetKeys::KeyIndex)
                .integer()
                .default(0)
                .not_null(),
        )
        .primary_key(
            Index::create()
                .table(SortingPresetKeys::Table)
                .col(SortingPresetKeys::PresetId)
                .col(SortingPresetKeys::KeyId),
        )
        .foreign_key(
            ForeignKey::create()
                .from(SortingPresetKeys::Table, SortingPresetKeys::PresetId)
                .to(SortingPresets::Table, SortingPresets::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .foreign_key(
            ForeignKey::create()
                .from(SortingPresetKeys::Table, SortingPresetKeys::KeyId)
                .to(SortKeys::Table, SortKeys::Id)
                .on_delete(ForeignKeyAction::Cascade),
        )
        .to_owned()
}

fn create_job_states() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(JobStates::Table)
        .col(
            ColumnDef::new(JobStates::JobType)
                .integer()
                .primary_key()
                .not_null(),
        )
        .col(ColumnDef::new(JobStates::Value).binary())
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
enum CdSourceMappings {
    Table,
    CdId,
    SourceId,
}

#[derive(Iden)]
enum SortingPresets {
    Table,
    Id,
}

#[derive(Iden)]
enum SortKeys {
    Table,
    Id,
    KeyType,
    Ascending,
    Value,
}

#[derive(Iden)]
enum SortingPresetKeys {
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
