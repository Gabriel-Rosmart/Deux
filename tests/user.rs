#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Extension, Router,
    };
    use deux::{
        api::auth::config::configure as auth, api::user::config::configure as user,
        db::mongo::Mongo,
    };

    use tower::ServiceExt;

    async fn app() -> Router {
        let db = Mongo::init().await.unwrap();

        let routes = Router::new().nest("/auth", auth()).nest("/user", user());

        Router::new().nest("/api", routes).layer(Extension(db))
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