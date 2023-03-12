#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use deux::{
        api::auth::config::configure as auth, api::profile::config::configure as profile,
        api::user::config::configure as user, db::mongo::Mongo,
    };
    use serde_json::json;
    use std::sync::Arc;
    use tower::ServiceExt;

    async fn app() -> Router {
        let db = Mongo::init().await.unwrap();
        let state = Arc::new(db);
        let routes = Router::new()
            .nest("/auth", auth())
            .nest("/user", user())
            .nest("/profile", profile());
        let app: Router<()> = Router::new().nest("/api", routes).with_state(state);
        app
    }

    #[tokio::test]
    async fn user_updates_correctly() {
        let app = app().await;
        let uri = "/api/profile/update";

        let auth_header = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6ImV4YW1wbGVAZ21haWwuY29tIn0.mvztvf67TauQ_3PIlM8YYmP0vb3d6yFE1olXVRUtlx0";

        let body = json!(
            {
                "password": "12345678"
            }
        );

        let response = app
            .oneshot(
                Request::put(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .header(http::header::AUTHORIZATION, auth_header)
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}