use sea_orm::entity::prelude::*;
 
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "filling")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}
 
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}
 
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }

    // fn queasb(&self)->
}
 
impl ActiveModelBehavior for ActiveModel {
}
