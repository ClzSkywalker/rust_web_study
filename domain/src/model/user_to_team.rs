use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user_to_team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
    pub deleted_at: DateTimeLocal,
    pub uid: String,
    pub tid: String,
    pub sort: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
    Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User=>Entity::belongs_to(super::user::Entity)
            .from(Column::Uid)
            .to(super::user::Column::Oc)
            .into(),
            Self::Team=>Entity::belongs_to(super::team::Entity)
            .from(Column::Uid)
            .to(super::team::Column::Oc)
            .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
