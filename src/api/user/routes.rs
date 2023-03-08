use axum::response::{ Response, IntoResponse };

pub async fn index() -> Response {
    "Protected route".into_response()
}