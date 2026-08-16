// src/health.rs
use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

/// Reports process liveness without requiring external credentials.
pub async fn health() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({ "status": "ok" })))
}
