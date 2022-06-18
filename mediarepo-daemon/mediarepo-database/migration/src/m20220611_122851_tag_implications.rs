use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220611_122851_tag_implications"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(create_tag_impliations_table()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TagImplications::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum TagImplications {
    Table,
    TagId,
    ImpliedTagId,
}

#[derive(Iden)]
enum Tags {
    Table,
    Id,
}

fn create_tag_impliations_table() -> TableCreateStatement {
    Table::create()
        .if_not_exists()
        .table(TagImplications::Table)
        .col(
            ColumnDef::new(TagImplications::TagId)
                .big_integer()
                .not_null(),
        )
        .col(
            ColumnDef::new(TagImplications::ImpliedTagId)
                .big_integer()
                .not_null(),
        )
        .primary_key(
            Index::create()
                .col(TagImplications::TagId)
                .col(TagImplications::ImpliedTagId),
        )
        .foreign_key(
            ForeignKey::create()
                .from(TagImplications::Table, TagImplications::TagId)
                .to(Tags::Table, Tags::Id),
        )
        .foreign_key(
            ForeignKey::create()
                .from(TagImplications::Table, TagImplications::ImpliedTagId)
                .to(Tags::Table, Tags::Id),
        )
        .to_owned()
}
