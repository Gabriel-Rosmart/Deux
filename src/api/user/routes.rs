use crate::{crypto::jwt::Claims, models::user::User, errors::server::AppError};
use axum::{
    response::{IntoResponse, Response},
    Extension, Json,
    extract::State
};
use std::sync::Arc;
use crate::shared::state::AppState;
use tokio::sync::Mutex;

pub async fn index(State(state): State<Arc<Mutex<AppState>>>, Extension(current_user): Extension<Claims>) -> Result<Response, AppError> {
    println!("{:#?}", current_user);
    //let collection = db.collection::<User>("users");
    let collection = state.lock().await.db.collection::<User>("users");
    let users = User::all(&collection).await?;
    Ok(Json(users).into_response())
}
