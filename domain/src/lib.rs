use migration::{Migrator, MigratorTrait};
use model::cake;
use rusqlite::Connection;
use sea_orm::{ColumnTrait, ConnectOptions, Database, DbConn, EntityTrait, QueryFilter,DatabaseConnection};
pub mod model;
pub mod entity;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

// const DATABASE_URL: &str = "sqlite:./db/event_shop.db"; // sqlite::memory:

pub async fn init_connection(path: &str) -> Result<DbConn, Box<dyn std::error::Error>> {
    match Connection::open(path) {
        Ok(_) => {}
        Err(r) => {
            common::log::log::error(r.to_string());
            return Err(Box::new(r));
        }
    }

    let db_path = "sqlite://".to_string() + path;
    let db: sea_orm::DatabaseConnection =
        match Database::connect(ConnectOptions::new(db_path)).await {
            Ok(r) => r,
            Err(r) => {
                common::log::log::error(r.to_string());
                return Err(Box::new(r));
            }
        };

    Migrator::up(&db, None).await?;

    let _a = cake::Entity::find()
        .filter(cake::Column::Name.like(""))
        .one(&db)
        .await;

    // Cake::Entity::find_by_id(11).all()

    // let t = cake::ActiveModel {
    //     title: Set(String::from("title")),
    //     text: Set(String::from("text")),
    //     oc: Set(String::from(common::ulid_genrate())),
    //     ..Default::default()
    // };

    // let _a = t.insert(&db).await?;

    Ok(db)
}
