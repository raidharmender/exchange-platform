use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[cfg(feature = "database")]
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[cfg(feature = "database")]
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Order book error: {0}")]
    OrderBook(String),
    
    #[error("Trade error: {0}")]
    Trade(String),
    
    #[error("User error: {0}")]
    User(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let (status_code, error_message) = match self {
            #[cfg(feature = "database")]
            AppError::Database(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Database error occurred".to_string(),
            ),
            #[cfg(feature = "database")]
            AppError::Redis(_) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Cache error occurred".to_string(),
            ),
            AppError::Authentication(msg) => (
                actix_web::http::StatusCode::UNAUTHORIZED,
                msg.clone(),
            ),
            AppError::Authorization(msg) => (
                actix_web::http::StatusCode::FORBIDDEN,
                msg.clone(),
            ),
            AppError::Validation(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::OrderBook(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Trade(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::User(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Internal(msg) => (
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                msg.clone(),
            ),
            AppError::NotFound(msg) => (
                actix_web::http::StatusCode::NOT_FOUND,
                msg.clone(),
            ),
            AppError::BadRequest(msg) => (
                actix_web::http::StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
        };

        HttpResponse::build(status_code).json(ErrorResponse {
            error: status_code.as_str().to_string(),
            message: error_message,
        })
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::Validation(errors.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        AppError::Authentication(format!("JWT error: {}", error))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(error: bcrypt::BcryptError) -> Self {
        AppError::Internal(format!("Password hashing error: {}", error))
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError::BadRequest(format!("JSON error: {}", error))
    }
} 