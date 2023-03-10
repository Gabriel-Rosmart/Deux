use axum::{
    response::IntoResponse,
    extract::Json
};
use hyper::StatusCode;
use validator::ValidationErrors;

pub enum AppError {
    InternalServerError,
    BadRequest(Json<ValidationErrors>),
}

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

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::BadRequest(e) => (StatusCode::BAD_REQUEST, e).into_response(),
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}