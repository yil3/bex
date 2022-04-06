pub mod oauth;
pub mod user;

use crate::handler::oauth::login;
use axum::routing::get;
use axum::{routing::post, Router};

use self::user::page;

pub fn authen_route() -> Router {
    Router::new().route("/login", post(login))
}

pub fn user_route() -> Router {
    Router::new()
        .route("/page", get(page))
}
