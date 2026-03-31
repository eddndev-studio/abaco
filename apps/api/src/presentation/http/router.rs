use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::application::commands;
use crate::application::ports::CreateAccountInput;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn build(pool: PgPool) -> Router {
    let state = AppState { db: pool };

    Router::new()
        .route("/health", get(health))
        .route("/api/v1/accounts", post(create_account))
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn create_account(
    State(_state): State<AppState>,
    Json(input): Json<CreateAccountInput>,
) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
    let account = commands::create_account(input)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({ "id": account.id, "code": account.code })),
    ))
}
