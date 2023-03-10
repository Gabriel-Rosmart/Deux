use crate::crypto::jwt::Claims;
use axum::{
    response::{IntoResponse, Response},
    Extension,
};

pub async fn index(Extension(current_user): Extension<Claims>) -> Response {
    println!("{:#?}", current_user);
    "Protected route".into_response()
}
