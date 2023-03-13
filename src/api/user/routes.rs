use crate::{crypto::jwt::Claims, models::user::User, errors::server::AppError};
use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
    extract::State
};
use std::sync::Arc;
use mongodb::Database;

pub async fn index(State(db): State<Arc<Database>>, Extension(current_user): Extension<Claims>) -> Result<Response, AppError> {
    println!("{:#?}", current_user);
    let collection = db.collection::<User>("users");
    let users = User::all(&collection).await?;
    Ok(Json(users).into_response())
}
