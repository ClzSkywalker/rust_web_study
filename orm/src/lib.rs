use migration::{Migrator, MigratorTrait};
use rusqlite::Connection;
use sea_orm::{ConnectOptions, Database, DbConn};
pub mod model;

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
    let db = match Database::connect(ConnectOptions::new(db_path)).await {
        Ok(r) => r,
        Err(r) => {
            common::log::log::error(r.to_string());
            return Err(Box::new(r));
        }
    };

    Migrator::up(&db, None).await?;

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
