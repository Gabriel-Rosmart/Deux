#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
        Router,
    };
    use deux::{
        api::auth::config::configure as auth, api::profile::config::configure as profile,
        api::user::config::configure as user, crypto::jwt::JWT, db::mongo::Mongo,
    };
    use hyper;
    use serde_json::json;
    use std::sync::Arc;
    use tower::{Service, ServiceExt};

    const AUTH_HEADER: &'static str = "Bearer eyJhbGciOiJIUzI1NiJ9.eyJfaWQiOnsiJG9pZCI6IjY0MDYzM2NkMTdlYTA0OWYzZTM3NGRlOSJ9LCJlbWFpbCI6ImV4YW1wbGVAZ21haWwuY29tIn0.z8Yeg_SI5l35f5anNUjAROVAaTP-coHAJ7UkpzwVsOA";

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

    /*** Login Tests ***/

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

        assert!(valid_token.is_ok());
    }

    #[tokio::test]
    async fn login_redirects_if_already_authenticated() {
        let app = app().await;
        let uri = "/api/auth/login";

        /*
            Note that a body is not neccesary, nor the Content-Type header
            since an already authenticated user should be redirected
        */

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::AUTHORIZATION, AUTH_HEADER)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);
    }

    /*** Register Tests ***/

    #[tokio::test]
    async fn register_checks_if_user_already_exists() {
        let app = app().await;
        let uri = "/api/auth/register";

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

        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[tokio::test]
    async fn register_user_correctly() {
        let mut app = app().await;

        let register_uri = "/api/auth/register";
        let login_uri = "/api/auth/login";
        let delete_profile_uri = "/api/profile/delete";

        let body = json!(
            {
                "email": "doesnotexists@gmail.com",
                "password": "12345678"
            }
        );

        /* Register user and check if all came up correctly */

        let register_request = Request::post(register_uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        let register_response = app
            .ready()
            .await
            .unwrap()
            .call(register_request)
            .await
            .unwrap();

        assert_eq!(register_response.status(), StatusCode::OK);

        /* Log in and check token is valid */

        let login_request = Request::post(login_uri)
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        let login_response = app
            .ready()
            .await
            .unwrap()
            .call(login_request)
            .await
            .unwrap();

        assert_eq!(login_response.status(), StatusCode::OK);

        let response_body = hyper::body::to_bytes(login_response.into_body())
            .await
            .unwrap();
        let response_string = String::from_utf8(response_body.to_vec()).unwrap();
        let valid_token = JWT::validate(&response_string);

        assert!(valid_token.is_ok());

        /* Delete the user to wrap around and prevent future failures */

        let delete_request = Request::delete(delete_profile_uri)
            .header(
                http::header::AUTHORIZATION,
                format!("Bearer {response_string}"),
            )
            .body(Body::empty())
            .unwrap();

        let delete_response = app
            .ready()
            .await
            .unwrap()
            .call(delete_request)
            .await
            .unwrap();

        assert_eq!(delete_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn register_redirects_if_already_authenticated() {
        let app = app().await;
        let uri = "/api/auth/register";

        /*
            Note that a body is not neccesary, nor the Content-Type header
            since an already authenticated user should be redirected
        */

        let response = app
            .oneshot(
                Request::post(uri)
                    .header(http::header::AUTHORIZATION, AUTH_HEADER)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FOUND);
    }
}
