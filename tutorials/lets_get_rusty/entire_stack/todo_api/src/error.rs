use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;

// Internal error type for the application
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] diesel::result::Error),

    #[error(transparent)]
    Pool(#[from] r2d2::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

// API error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Internal(String),
}

impl From<crate::error::Error> for ApiError {
    // Convert internal Error to ApiError
    fn from(e: crate::error::Error) -> Self {
        ApiError::Internal(e.to_string())
    }
}

// Implement Responder for ApiError to convert it into HTTP responses
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let (status, msg) = match self {
            ApiError::NotFound(m) => (Status::NotFound, m),
            ApiError::Internal(m) => (Status::InternalServerError, m),
        };

        Response::build_from(Json(ErrorResponse { message: msg }).respond_to(req)?)
            .status(status)
            .ok()
    }
}
