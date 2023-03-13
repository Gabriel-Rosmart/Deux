use crate::{crypto::jwt::Claims, extractors::profile::UpdateProfileRequest};
use crate::errors::server::AppError;
use crate::models::user::User;
use crate::constants::messages::DatabaseMessages;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use mongodb::bson::Document;
use validator::Validate;
use std::sync::Arc;
use crate::shared::state::AppState;
use tokio::sync::Mutex;

pub async fn update(
    State(state): State<Arc<Mutex<AppState>>>,
    Extension(current_user): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>
) -> Result<Response, AppError> {
    payload.validate()?;
    //let collection = db.collection::<Document>("users");
    let collection = state.lock().await.db.collection::<Document>("users");
    let result = User::update(&collection, &current_user.email(), payload).await?;
    if result.modified_count == 0 {
        return Ok((StatusCode::NOT_FOUND, DatabaseMessages::NOT_EXISTS).into_response());
    }
    Ok(StatusCode::OK.into_response())
}

pub async fn delete(
    State(state): State<Arc<Mutex<AppState>>>,
    Extension(current_user): Extension<Claims>,
) -> Result<Response, AppError> {

    //let collection = db.collection::<Document>("users");
    let collection = state.lock().await.db.collection::<Document>("users");
    let result = User::delete(&collection, &current_user.email()).await?;
    if result.deleted_count == 0 {
        return Ok((StatusCode::NOT_FOUND, DatabaseMessages::NOT_EXISTS).into_response());
    }
    Ok(StatusCode::OK.into_response())
}
