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
use validator::Validate;

use crate::extractors::login::JsonLogin;

pub async fn index(Extension(db): Extension<Database>, Json(payload): Json<JsonLogin>) -> Response {
    let _coll = db.collection::<Document>("users");
    let is_valid = payload.validate();
    

    match is_valid {
        Ok(_) => return StatusCode::OK.into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
    }
}