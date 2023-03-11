use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension
};
use std::sync::Arc;
use mongodb::Database;
use crate::errors::server::AppError;
use crate::crypto::jwt::Claims;

pub async fn delete(State(db): State<Arc<Database>>, Extension(current_user): Extension<Claims>) -> Result<Response, AppError> {
    println!("{:#?}", current_user);
    Ok(StatusCode::OK.into_response())
}