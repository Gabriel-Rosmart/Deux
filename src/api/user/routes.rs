use axum::{
    Extension,
    response::{ Response, IntoResponse }
};
use crate::crypto::jwt::Claims;

pub async fn index(Extension(current_user): Extension<Claims>) -> Response {
    println!("{:#?}", current_user);
    "Protected route".into_response()
}