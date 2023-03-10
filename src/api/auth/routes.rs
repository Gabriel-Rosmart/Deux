use std::sync::Arc;

use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use mongodb::{bson::Document, Database};

use crate::crypto::jwt::JWT;

use crate::extractors::auth::{LoginRequest, RegisterRequest};
use crate::models::user::User;
use validator::Validate;

use crate::errors::server::AppError;

pub async fn login(Extension(db): Extension<Arc<Database>>, Json(payload): Json<LoginRequest>) -> Result<Response, AppError> {
    let collection = db.collection::<User>("users");

    /* Validate given json, return BAD_REQUEST with error if unvalid */

    payload.validate()?;
    /*
    if let Err(error) = is_valid {
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }*/

    /* Verify user and password, return INTERNAL_SERVER_ERROR if db communication fails */

    let exists = User::verify(collection, (payload.email, payload.password)).await?;
    /*
    if exists.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }*/

    let cl = JWT::generate();

    if exists {
        Ok((StatusCode::OK, cl).into_response())
    } else {
        Ok(StatusCode::UNAUTHORIZED.into_response())
    }
}

pub async fn register(
    Extension(db): Extension<Arc<Database>>,
    Json(payload): Json<RegisterRequest>,
) -> Response {
    let collection = db.collection::<Document>("users");

    /* Validate given json, return BAD_REQUEST with error if unvalid */

    let is_valid = payload.validate();

    if let Err(error) = is_valid {
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    /* Create the user, return INTERNAL_SERVER_ERROR if db communication fails  */

    let created = User::create(collection, (payload.email, payload.password)).await;

    match created {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
