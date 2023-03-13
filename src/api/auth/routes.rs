use std::sync::Arc;

use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use mongodb::bson::Document;

use crate::crypto::jwt::{Claims, JWT};
use crate::constants::messages::DatabaseMessages;

use crate::extractors::auth::{LoginRequest, RegisterRequest};
use crate::models::user::User;
use validator::Validate;

use crate::errors::server::AppError;

use crate::shared::state::AppState;
use tokio::sync::Mutex;

pub async fn login(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Response, AppError> {
    //let collection = state.db.collection::<User>("users");

    let collection = state.lock().await.db.collection::<User>("users");

    /* Validate given json, return BAD_REQUEST with error if unvalid */

    payload.validate()?;

    /* Verify user and password, return INTERNAL_SERVER_ERROR if db communication fails */

    let (exists, id) = User::verify(&collection, (&payload.email, &payload.password)).await?;


    if exists {
        let claims = Claims::new(id.unwrap(), &payload.email);
        let cl = JWT::generate(claims)?;
        Ok((StatusCode::OK, cl).into_response())
    } else {
        Ok(StatusCode::UNAUTHORIZED.into_response())
    }
}

pub async fn register(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Response, AppError> {
    //let collection = db.collection::<Document>("users");

    let collection = state.lock().await.db.collection::<Document>("users");

    /* Validate given json, return BAD_REQUEST with error if unvalid */

    payload.validate()?;

    /* Check if user already exists */

    let exists = User::exists(&collection, &payload.email).await?;

    if exists {
        return Ok((StatusCode::CONFLICT, DatabaseMessages::ALREADY_EXISTS).into_response());
    }

    /* Create the user, return INTERNAL_SERVER_ERROR if db communication fails  */

    User::create(&collection, (&payload.email, &payload.password)).await?;

    Ok(StatusCode::OK.into_response())
}
