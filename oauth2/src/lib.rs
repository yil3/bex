#![allow(non_snake_case)]

use axum::routing::get;
use crate::handler::authen_route;
use crate::models::token::Claims;
use axum::extract::extractor_middleware;
use axum::extract::Extension;
use axum::Router;
use handler::user_route;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;
use sqlx::Row;

pub mod handler;
pub mod models;
pub mod repository;
pub mod service;

pub mod middleware;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref POOL: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&env::var("DATABASE_URL").unwrap())
        .unwrap();
}

const SECRET: &[u8] = b"deadbeef";

pub async fn init_db_pool(app: Router) -> Router {
    app.layer(Extension(&*POOL))
}

pub async fn init_router(app: Router) -> Router {
    app.nest("/user", user_route())
        .layer(extractor_middleware::<Claims>())
        .nest("/oauth", authen_route())
        .route("/test", get(test))
}

pub async fn test() {
    let mut sql = String::new();
    sql.push_str("select count(0) from sys_user");
    let a = sqlx::query(&sql).fetch_one(&*POOL).await.unwrap();
    println!("{:#?}",a.column("count"));
    println!("{:#?}",a.get::<i64, _>("count"));
}
