use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Cake::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Cake::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Cake::Name).string())
                    .to_owned(),
            )
            .await
            .expect("create cake");

        manager
            .create_table(
                Table::create()
                    .table(Fruit::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Fruit::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Fruit::Name).string())
                    .col(ColumnDef::new(Fruit::CakeId).integer())
                    .to_owned(),
            )
            .await
            .expect("create fruit");

        manager
            .create_table(
                Table::create()
                    .table(CakeFilling::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(CakeFilling::FillingId).integer())
                    .col(ColumnDef::new(CakeFilling::CakeId).integer())
                    .to_owned(),
            )
            .await
            .expect("create cake_filling");

        manager
            .create_table(
                Table::create()
                    .table(Filling::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Filling::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Filling::Name).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Cake::Table).to_owned())
            .await
            .expect("drop cake");
        manager
            .drop_table(Table::drop().table(Fruit::Table).to_owned())
            .await
            .expect("drop fruit");
        manager
            .drop_table(Table::drop().table(CakeFilling::Table).to_owned())
            .await
            .expect("drop cake_filling");
        manager
            .drop_table(Table::drop().table(Filling::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Cake {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Fruit {
    Table,
    Id,
    Name,
    CakeId,
}

#[derive(Iden)]
enum CakeFilling {
    Table,
    CakeId,
    FillingId,
}

#[derive(Iden)]
enum Filling {
    Table,
    Id,
    Name,
}
