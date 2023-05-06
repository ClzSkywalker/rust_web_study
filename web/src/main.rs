use std::{net::SocketAddr, str::FromStr};

use axum::{Router, Server};
use sea_orm::*;

#[tokio::main]
async fn main() {
    common::log::log::init_log("./logs");
    let db = orm::init().await.unwrap();

    let state = domain::AppState { conn: db };

    let app = Router::new()
        .route("/user", get(api::api::user::user_find_by_oc))
        .with_state(state);

    let addr = SocketAddr::from_str("127.0.0.1:7096").unwrap();
    Server::bind(&addr).serve(app.into_make_service()).await?;
    Ok(())
}
