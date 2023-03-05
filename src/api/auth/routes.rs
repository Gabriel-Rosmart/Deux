use axum::{
    Extension,
    extract::Json, 
    response::{ Response, IntoResponse },
    http::StatusCode
};
use mongodb::{
    Database,
    bson::Document
};

use crate::extractors::auth::{ JsonLogin, JsonRegister };
use crate::models::user::User;
use validator::Validate;

pub async fn login(Extension(db): Extension<Database>, Json(payload): Json<JsonLogin>) -> Response {
    let _coll = db.collection::<Document>("users");
    let is_valid = payload.validate();
    

    match is_valid {
        Ok(_) => return StatusCode::OK.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}

pub async fn register(Extension(db): Extension<Database>, Json(payload): Json<JsonRegister>) -> Response {
    let collection = db.collection::<Document>("users");
    let is_valid = payload.validate();

    if let Err(error) = is_valid {
        return  (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    let created = User::create(collection, (payload.email, payload.password)).await;

    match created {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}