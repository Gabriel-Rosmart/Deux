use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    InternalServerError,
    BadRequest(Json<ValidationErrors>),
}

/*** Implementations for coverting error allowing us to use the ? operator ***/

impl From<ValidationErrors> for AppError {
    fn from(value: ValidationErrors) -> Self {
        Self::BadRequest(Json(value))
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(_: mongodb::error::Error) -> Self {
        Self::InternalServerError
    }
}

impl From<jwt::Error> for AppError {
    fn from(_: jwt::Error) -> Self {
        Self::InternalServerError
    }
}

impl From<hmac::digest::InvalidLength> for AppError {
    fn from(_: hmac::digest::InvalidLength) -> Self {
        Self::InternalServerError
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
