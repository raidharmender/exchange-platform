use actix_web::{get, HttpResponse, Responder};
use serde::Serialize;
use chrono::Utc;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: String,
    version: String,
}

#[get("/health")]
pub async fn health_check() -> impl Responder {
    let health = HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    HttpResponse::Ok().json(health)
} 