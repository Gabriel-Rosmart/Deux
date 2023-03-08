#[cfg(test)]
mod tests {
    use axum::{
        Router,
        Extension,
        body::Body,
        http::{self, Request, StatusCode}
    };
    use serde_json::json;
    use tower::ServiceExt;
    use deux::{
        db::mongo::Mongo,
        api::auth::config::configure as auth,
        api::user::config::configure as user
    };
    
    async fn app() -> Router {
        let db = Mongo::init().await.unwrap();
    
        let routes = Router::new().nest("/auth", auth()).nest("/user", user());
    
        Router::new().nest("/api", routes).layer(Extension(db))
    }

    #[tokio::test]
    async fn login_does_not_accept_empty_body() {
        let app = app().await;
        let uri = "/api/auth/login";

        let response = app
            .oneshot(Request::post(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn login_does_not_accept_malformed_body() {
        let app = app().await;
        let uri = "/api/auth/login";

        /* Note that the body is should have 'password' and not 'passwo' */
        let malformed_body = json!(
            {
                "email": "example@gmail.com",
                "passwo": "1234"
            }
        );

        let response = app
            .oneshot(Request::post(uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(malformed_body.to_string())).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
