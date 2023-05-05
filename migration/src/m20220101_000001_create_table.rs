use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(User::Oc).string().unique_key().not_null())
                    .col(ColumnDef::new(User::TeamIdPort).string().not_null())
                    .col(ColumnDef::new(User::NickName).string())
                    .col(ColumnDef::new(User::MemberType).integer())
                    .col(ColumnDef::new(User::RegisterType).integer())
                    .col(ColumnDef::new(User::Picture).string())
                    .col(ColumnDef::new(User::Email).string())
                    .col(ColumnDef::new(User::Phone).string())
                    .col(ColumnDef::new(User::Pwd).string())
                    .col(ColumnDef::new(User::Version).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(Team::Oc).string().unique_key().not_null())
                    .col(ColumnDef::new(Team::CreatedBy).string())
                    .col(ColumnDef::new(Team::Name).string())
                    .col(ColumnDef::new(Team::Description).string())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserToTeam::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Base::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Base::CreatedAt).date_time())
                    .col(ColumnDef::new(Base::UpdatedAt).date_time())
                    .col(ColumnDef::new(Base::DeletedAt).date_time())
                    .col(ColumnDef::new(UserToTeam::Uid).string())
                    .col(ColumnDef::new(UserToTeam::Tid).string())
                    .col(ColumnDef::new(UserToTeam::Sort).integer())
                    .index(Index::create().unique().name("udx_utt_uid_tid").col(UserToTeam::Uid).col(UserToTeam::Tid))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        manager.drop_table(Table::drop().table(UserToTeam::Table).to_owned()).await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Base {
    Id,
    CreatedAt,
    UpdatedAt,
    DeletedAt,
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum User {
    Table,
    Oc,
    TeamIdPort,
    NickName,
    MemberType,
    RegisterType,
    Picture,
    Email,
    Phone,
    Pwd,
    Version,
}

#[derive(Iden)]
enum Team {
    Table,
    Oc,
    CreatedBy,
    Name,
    Description,
}

#[derive(Iden)]
enum UserToTeam {
    Table,
    Uid,
    Tid,
    Sort,
}
