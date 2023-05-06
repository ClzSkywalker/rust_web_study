use domain::model::*;
use sea_orm::*;
pub struct UserDao<'a> {
    db: &'a DatabaseConnection,
}
pub fn new(db: &DatabaseConnection) -> UserDao {
    UserDao { db }
}

impl<'a> UserDao<'a> {
    pub async  fn find_by_os(&self, oc: String) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Oc.eq(oc))
            .one(self.db)
            .await
    }

    pub async fn insert(&self, u: user::ActiveModel) -> Result<InsertResult<user::ActiveModel>, DbErr> {
        user::Entity::insert(u).exec(self.db).await
    }
}
