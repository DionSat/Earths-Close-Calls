use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::ParseError;
use http::StatusCode;
use reqwest::Error as ReqwestError;
use serde_json::{json, Error as SerdeError};
use sqlx::Error;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    MissingCredentials,
    InvalidPassword,
    UserDoesNotExist,
    UserAlreadyExists,
    InvalidDate(chrono::ParseError),
    RequestAPI(ReqwestError),
    SerdeFailedParse(SerdeError),
    InvalidToken,
    InternalServerError,
    #[allow(dead_code)]
    Any(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::Any(err) => {
                let message = format!("Internal server error! {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, message)
            }
            AppError::MissingCredentials => (
                StatusCode::UNAUTHORIZED,
                "Your credentials were missing or otherwise incorrect".to_string(),
            ),
            AppError::UserDoesNotExist => (
                StatusCode::UNAUTHORIZED,
                "Your account does not exist!".to_string(),
            ),
            AppError::UserAlreadyExists => (
                StatusCode::UNAUTHORIZED,
                "There is already an account with that email address in the system".to_string(),
            ),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid Token".to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid Password".to_string()),
            AppError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something terrible happened".to_string(),
            ),
            AppError::InvalidDate(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::RequestAPI(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
            AppError::SerdeFailedParse(err) => (StatusCode::SERVICE_UNAVAILABLE, err.to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::Database(value)
    }
}

impl From<chrono::ParseError> for AppError {
    fn from(value: ParseError) -> Self {
        AppError::InvalidDate(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: ReqwestError) -> Self {
        AppError::RequestAPI(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: SerdeError) -> Self {
        AppError::SerdeFailedParse(value)
    }
}
