#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use deux::{
        api::auth::config::configure as auth, api::user::config::configure as user,
        db::mongo::Mongo,
    };
    use std::sync::Arc;
    use tower::ServiceExt;

    async fn app() -> Router {

        let db = Mongo::init().await.unwrap();

        let state = Arc::new(db);

        let routes = Router::new().nest("/auth", auth()).nest("/user", user());
    
        let app: Router<()> = Router::new().nest("/api", routes).with_state(state);

        app
    }

    #[tokio::test]
    async fn middleare_validates_correct_token() {
        let app = app().await;
        let uri = "/api/user";

        let auth_header = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InRlc3RAZ21haWwuY29tIn0.wSwj67wG9ZUYAVVBnma-SIeSK9wLGGuZNSlMzlQiTQ0";

        let response = app
            .oneshot(
                Request::get(uri)
                    .header(http::header::AUTHORIZATION, auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}