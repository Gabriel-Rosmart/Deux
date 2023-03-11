#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use deux::{
        api::auth::config::configure as auth, api::user::config::configure as user,
        crypto::jwt::JWT, db::mongo::Mongo,
    };
    use std::sync::Arc;
    use hyper;
    use serde_json::json;
    use tower::ServiceExt;

    async fn app() -> Router {
        let db = Mongo::init().await.unwrap();
        let state = Arc::new(db);
        let routes = Router::new().nest("/auth", auth()).nest("/user", user());
        let app: Router<()> = Router::new().nest("/api", routes).with_state(state);
        app
    }

    #[tokio::test]
    async fn login_does_not_accept_empty_body() {
        let app = app().await;
        let uri = "/api/auth/login";

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::empty())
                    .unwrap(),
            )
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
            .oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(malformed_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn login_validates_correct_email() {
        let app = app().await;
        let uri = "/api/auth/login";

        let bad_email_body = json!(
            {
                "email": "notanemail",
                "password": "12345678"
            }
        );

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(bad_email_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn login_yields_unathorized_for_inexsistent_user() {
        let app = app().await;
        let uri = "/api/auth/login";

        let inexistent_user_body = json!(
            {
                "email": "doesnotexists@gmail.com",
                "password": "12345678"
            }
        );

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(inexistent_user_body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn login_responds_with_valid_token() {
        let app = app().await;
        let uri = "/api/auth/login";

        let body = json!(
            {
                "email": "example@gmail.com",
                "password": "12345678"
            }
        );

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let response_body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_string = String::from_utf8(response_body.to_vec()).unwrap();
        let valid_token = JWT::validate(&response_string);

        assert_eq!(valid_token.is_ok(), true);
    }

    #[tokio::test]
    async fn login_redirects_if_already_authenticated() {
        let app = app().await;
        let uri = "/api/auth/login";

        /*
            Note that a body is not neccesary, nor the Content-Type header
            since an already authenticated user should be redirected
        */

        let auth_header = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InRlc3RAZ21haWwuY29tIn0.wSwj67wG9ZUYAVVBnma-SIeSK9wLGGuZNSlMzlQiTQ0";

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::AUTHORIZATION, auth_header)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);
    }
}
