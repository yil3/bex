use axum::Json;
use axum::extract::Query;
use axum::response::IntoResponse;
use common::models::resp::R;
use std::collections::HashMap;
use crate::service::user;

pub async fn page(params: Query<HashMap<String, String>>) -> impl IntoResponse {
    let page = user::Service.page(&params.0).await;
    Json(R::success(page))
}

