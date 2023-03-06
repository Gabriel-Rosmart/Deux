use axum::{
    extract::Json,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use mongodb::{bson::Document, Database};

use crate::extractors::auth::{JsonLogin, JsonRegister};
use crate::models::user::User;
use validator::Validate;

pub async fn login(Extension(db): Extension<Database>, Json(payload): Json<JsonLogin>) -> Response {
    let collection = db.collection::<User>("users");

    /* Validate given json, return BAD_REQUEST with error if unvalid */

    let is_valid = payload.validate();

    if let Err(error) = is_valid {
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    /* Verify user and password, return INTERNAL_SERVER_ERROR if db communication fails */

    let exists = User::verify(collection, (payload.email, payload.password)).await;

    if exists.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if exists.unwrap() {
        StatusCode::OK.into_response()
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

pub async fn register(
    Extension(db): Extension<Database>,
    Json(payload): Json<JsonRegister>,
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
