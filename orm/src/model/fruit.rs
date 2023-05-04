use sea_orm::entity::prelude::*;
 
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "fruit")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub cake_id:i32
}
 
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Cake,
}
 
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self{
            Self::Cake=>Entity::belongs_to(super::cake::Entity)
            .from(Column::CakeId)
            .to(super::cake::Column::Id)
            .into(),
        }
    }

}

impl Related<super::cake::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cake.def()
    }
}
 
impl ActiveModelBehavior for ActiveModel {
}

