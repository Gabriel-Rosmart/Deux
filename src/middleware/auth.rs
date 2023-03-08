use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct Auth;

impl<S> Layer<S> for Auth {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
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
        /* Check that Authorization header is all correct */
        let token = extract_bearer(&request);
        if let Err(error) = token {
            return Box::pin(async move { Ok((StatusCode::UNAUTHORIZED, error).into_response()) });
        }

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            //let response = StatusCode::FOUND.into_response();
            Ok(response)
        })
    }
}

fn extract_bearer(request: &Request<Body>) -> Result<String, &'static str> {
    /* Check if Authorization header is present */
    let header = request.headers().get("Authorization");
    if header.is_none() {
        return Err("No Authorization header present on request");
    }

    /* Check if Bearer is prensent on Authorization header */
    let inner_string = String::from_utf8(header.unwrap().as_bytes().to_owned()).unwrap();
    /*
    has_bearer(&inner_string)?;
    has_whitespace(&inner_string)?;*/

    if !validate_header(&inner_string) { return Err("Malformed auth header"); }

    let token = has_token(&inner_string)?;

    Ok(token)
}


fn validate_header(header: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Bearer [a-zA-z0-9]+").unwrap();
    }
    RE.is_match(header)
}
/*
fn has_bearer(header: &str) -> Result<(), &'static str> {
    let bearer = header.get(0..6).map(|value| value.to_string());
    if bearer.is_none() || bearer.unwrap_or("".to_string()) != "Bearer" {
        return Err("No Bearer present on Authorization header");
    }
    Ok(())
}

fn has_whitespace(header: &str) -> Result<(), &'static str> {
    if header.get(6..7).unwrap_or("").to_string() != " " {
        return Err("Malformed Authorization header");
    }
    Ok(())
}
*/
fn has_token(header: &str) -> Result<String, &'static str> {
    let token = header.get(7..).map(|value| value.to_string());
    if token.is_none() {
        return Err("No Bearer present on Authorization header");
    }
    Ok(token.unwrap())
}