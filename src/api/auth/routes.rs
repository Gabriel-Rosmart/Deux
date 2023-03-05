use axum::{
    Extension,
    extract::Json
};
use mongodb::{
    Database,
    bson::Document
};

use crate::extractors::login::JsonLogin;

pub async fn index(Extension(db): Extension<Database>, Json(payload): Json<JsonLogin>) -> &'static str {
    let _coll = db.collection::<Document>("users");
    println!("{:#?}", payload);
    "Hello, World!"
}