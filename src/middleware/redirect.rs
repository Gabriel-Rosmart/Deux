use super::helpers::bearer::extract_bearer;
use crate::crypto::jwt::JWT;
use crate::constants::messages::TokenMessages;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct Redirect;

impl<S> Layer<S> for Redirect {
    type Service = RedirectMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RedirectMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct RedirectMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for RedirectMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        if request.headers().get("Authorization").is_some() {
            /* Check that Authorization header is all correct */
            let token = extract_bearer(&request);

            if let Err(error) = token {
                return Box::pin(
                    async move { Ok((StatusCode::UNAUTHORIZED, error).into_response()) },
                );
            }

            /* Check if token is valid */
            let current_user = JWT::validate(&token.unwrap());

            if current_user.is_err() {
                return Box::pin(async move {
                    Ok((StatusCode::UNAUTHORIZED, TokenMessages::INVALID).into_response())
                });
            }

            return Box::pin(async move { Ok(StatusCode::FOUND.into_response()) });
        }

        //request.extensions_mut().insert(current_user.unwrap());
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}
