use axum::{http::Uri, routing::get, Router};
use config::AppConfig;
use state::AppState;
use tokio::net::TcpListener;
use utils::init_tracing;

mod config;
mod error;
mod state;
mod utils;

async fn run(state: AppState, listener: TcpListener) -> Result<(), anyhow::Error> {
    let router = Router::new()
        .route("/", get(|| async { "Hello from postgresql-in-action!" }))
        .fallback(fallback);

    let server = axum::serve(listener, router);

    Ok(server.await?)
}

async fn fallback(uri: Uri) -> (axum::http::StatusCode, String) {
    tracing::warn!("No route for {uri}");
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route for {uri}"),
    )
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    init_tracing();

    let config = AppConfig::load().expect("Failed to load config");
    let state = AppState::new(config)
        .await
        .expect("Failed to init AppState");

    let port = 3000;
    let url = format!("0.0.0.0:{}", port);
    tracing::info!("Running at http://{url}");
    let listener = TcpListener::bind(url).await?;

    let rows = sqlx::query_file!("queries/get_movie_with_limit.sql", 3)
        .fetch_all(state.db())
        .await?;
    dbg!(rows);

    run(state, listener).await?;

    Ok(())
}
