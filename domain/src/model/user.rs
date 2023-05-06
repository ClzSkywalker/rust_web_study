use sea_orm::entity::prelude::*;
use serde::{Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel,Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub created_at: DateTimeLocal,
    pub updated_at: DateTimeLocal,
    pub deleted_at: DateTimeLocal,
    pub oc:String,
    pub team_id_port:String,
    pub nick_name: String,
    pub member_type:MemberType,
    pub register_type:RegisterType,
    pub email:String,
    pub phone:String,
    pub pwd:String,
    pub version:String
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum,Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum MemberType{
    #[sea_orm(num_value=0)]
    Normal,
    #[sea_orm(num_value=1)]
    Member,
    #[sea_orm(num_value=2)]
    Permanent
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum,Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum RegisterType{
    #[sea_orm(num_value=1)]
    Email,
    #[sea_orm(num_value=2)]
    Phone,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    // Team,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("")
        // match self {
        //     Self::Team=>Entity::has_one(super::team::Entity).into(), 
        // }
    }
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        // Relation::Team.def()
        super::user_to_team::Relation::Team.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_to_team::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}