use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "team")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
    pub deleted_at: DateTimeLocal,
    pub oc: String,
    pub created_by: String,
    pub name: String,
    pub description: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::user::Entity)
                .from(Column::Oc)
                .to(super::user::Column::Oc)
                .into(),
        }
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        // Relation::User.def()
        super::user_to_team::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_to_team::Relation::Team.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
