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
    let collection = state.lock().await.db.collection::<User>("users");

    let has = state.lock().await.cache.get("users");

    if has.is_some() {
        return Ok(Json(has.unwrap()).into_response());
    }

    //if has.is_some() { return Ok(Json(has.unwrap().clone()).into_response())}

    let users = User::all(&collection).await?;
    state.lock().await.cache.insert("users".to_string(), users.clone());
    Ok(Json(users).into_response())
}
