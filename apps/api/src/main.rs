mod domain;
mod application;
mod infrastructure;
mod presentation;

use infrastructure::config::Settings;
use presentation::http::router;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = Settings::load();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&settings.database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let app = router::build(pool.clone())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = format!("{}:{}", settings.host, settings.port);
    tracing::info!("Abaco API listening on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
