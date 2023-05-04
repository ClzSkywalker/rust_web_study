use sea_orm::entity::prelude::*;
 
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "cake_filling")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub cake_id:i32,
    pub filling_id: i32,
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