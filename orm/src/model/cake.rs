use sea_orm::entity::prelude::*;
 
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cake")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}
 
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Fruit
}
 
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self{
            Self::Fruit=>Entity::has_many(super::fruit::Entity).into(),
        }
    }
}

impl Related<super::fruit::Entity> for Entity{
    fn to() -> RelationDef {
        Relation::Fruit.def()
    }
}
 
impl ActiveModelBehavior for ActiveModel {
}