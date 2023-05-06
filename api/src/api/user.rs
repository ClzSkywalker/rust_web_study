use axum::{
    extract::{Path, State},
    Json,
};

pub async fn user_find_by_oc(
    state: State<domain::AppState>,
    Path(oc): Path<String>,
) -> Json<domain::model::user::Entity> {
    let u = orm::infra::user::new(&state.conn);
    let r = match u.find_by_os(oc).await {
        Ok(r) => match r {
            Some(r1) => r1,
            None => return Json(""),
        },
        Err(r) => {
            common::log::log::error(r.to_string());
            return Json("");
        }
    };
    Json(r)
}
