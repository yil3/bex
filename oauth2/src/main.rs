use oauth2::init_db_pool;
use oauth2::init_router;
use axum::Router;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app = Router::new();
    let app = init_router(app).await;
    let app = init_db_pool(app).await;

    tracing::debug!("listening on {}", ":3030");

    axum::Server::bind(&"127.0.0.1:3030".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server error");
}
