use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension
};
use std::sync::Arc;
use mongodb::{Database, bson::Document};
use crate::errors::server::AppError;
use crate::crypto::jwt::Claims;
use crate::models::user::User;

pub async fn delete(State(db): State<Arc<Database>>, Extension(current_user): Extension<Claims>) -> Result<Response, AppError> {
    println!("{:#?}", current_user);
    let collection = db.collection::<Document>("users");
    let result = User::delete(&collection, &current_user.email()).await?;
    if result.deleted_count == 0 { return Ok((StatusCode::NOT_FOUND, "user does not exists").into_response());}
    Ok(StatusCode::OK.into_response())
}