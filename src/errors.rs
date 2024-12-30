use std::fmt;

use actix_web::{error::BlockingError, HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};
use log::{error, info, warn};
use validator::{Validate, ValidationErrors};

// Enhanced error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug)]
pub enum TaskError {
    NotFound(String),
    ValidationError(ValidationErrors),
    DatabaseError(String),
    BlockingError(BlockingError),
    InternalError(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(msg) => write!(f, "Not found: {}", msg),
            TaskError::ValidationError(err) => write!(f, "Validation error: {:?}", err),
            TaskError::DatabaseError(err) => write!(f, "Database error: {}", err),
            TaskError::BlockingError(err) => write!(f, "Blocking error: {}", err),
            TaskError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        match self {
            TaskError::NotFound(msg) => {
                warn!("Resource not found: {}", msg);
                HttpResponse::NotFound().json(ErrorResponse {
                    code: "NOT_FOUND".into(),
                    message: msg.clone(),
                    details: None,
                })
            }
            TaskError::ValidationError(err) => {
                warn!("Validation error: {:?}", err);
                HttpResponse::BadRequest().json(ErrorResponse {
                    code: "VALIDATION_ERROR".into(),
                    message: "Invalid input data".into(),
                    details: Some(serde_json::to_value(err.errors()).unwrap_or_default()),
                })
            }
            TaskError::DatabaseError(err) => {
                error!("Database error: {}", err);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: "DATABASE_ERROR".into(),
                    message: "Database operation failed".into(),
                    details: None,
                })
            }
            TaskError::BlockingError(err) => {
                error!("Blocking error: {}", err);
                HttpResponse::ServiceUnavailable().json(ErrorResponse {
                    code: "SERVICE_UNAVAILABLE".into(),
                    message: "Service temporarily unavailable".into(),
                    details: None,
                })
            }
            TaskError::InternalError(msg) => {
                error!("Internal server error: {}", msg);
                HttpResponse::InternalServerError().json(ErrorResponse {
                    code: "INTERNAL_ERROR".into(),
                    message: "An unexpected error occurred".into(),
                    details: None,
                })
            }
        }
    }
}
