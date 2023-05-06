use std::error::Error;

pub mod infra;
pub mod service;

pub async fn init() -> Result<sea_orm::DatabaseConnection, Box<dyn Error>> {
    let r = match domain::init_connection("./event_shop.db").await {
        Ok(r) => r,
        Err(r) => {
            println!("db err:{}", r);
            common::log::log::error(r.to_string());
            return Err(r);
        }
    };
    Ok(r)
}
